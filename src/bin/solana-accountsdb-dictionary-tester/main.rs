use std::{
    collections::HashMap,
    fs::File,
    path::PathBuf,
    str::FromStr,
    time::{Duration, Instant},
};

use clap::Parser;
use itertools::Itertools;
use lz4::block::CompressionMode;
use solana_accountsdb_compression_dictionary_utils::{
    append_vec_iter, archived::ArchiveSnapshotExtractor,
    partial_pubkey_by_bits::PartialPubkeyByBits, SnapshotExtractor,
};

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'a', long)]
    pub snapshot_archive_path: String,

    #[arg(short = 'd', long)]
    pub dictionary: Option<String>,

    #[arg(short = 's', long)]
    pub lz4_compression: Option<i32>,

    #[arg(short = 'm', long)]
    pub max_number_of_accounts: Option<u64>,
}

type DictionaryMap = HashMap<PartialPubkeyByBits, Vec<u8>>;

pub fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let args = Args::parse();

    println!("tester args : {:?}", args);

    let Args {
        snapshot_archive_path,
        dictionary,
        lz4_compression,
        max_number_of_accounts,
    } = args;
    // loading dictionary
    let dictionary = match dictionary {
        Some(dictionary_path) => {
            let dictionary_binary = std::fs::read(dictionary_path).unwrap();
            let dictionary = bincode::deserialize::<DictionaryMap>(&dictionary_binary).unwrap();
            drop(dictionary_binary);
            dictionary
        }
        None => DictionaryMap::new(),
    };
    let any_key_in_dict = dictionary.keys().find_or_first(|_| true); // get any key
    let nb_bits = any_key_in_dict.map(|x| x.nb_bits).unwrap_or_default();

    let archive_path = PathBuf::from_str(snapshot_archive_path.as_str()).unwrap();

    let mut loader: ArchiveSnapshotExtractor<File> =
        ArchiveSnapshotExtractor::open(&archive_path).unwrap();

    let mut total_size_uncompressed: usize = 0;
    let mut total_size_compressed: usize = 0;
    let mut accounts_with_dict: usize = 0;
    let mut accounts_without_dict: usize = 0;
    let mut compression_errors: usize = 0;
    let mut decompression_errors: usize = 0;
    let mut time_compression: Duration = Duration::from_micros(0);
    let mut time_decompression: Duration = Duration::from_micros(0);
    let mut account_total: u64 = 0;
    let mut compress_out_buffer = vec![0; 16 * 1024 * 1024]; // 16 MB buffer

    let max_number_of_accounts = max_number_of_accounts.unwrap_or(u64::MAX);
    for vec in loader.iter() {
        let append_vec = vec.unwrap();
        // info!("size: {:?}", append_vec.len());
        for handle in append_vec_iter(&append_vec) {
            let stored = handle.access().unwrap();
            if stored.meta.data_len < 64 {
                continue;
            }

            let owner_partial_pubkey_by_bits =
                PartialPubkeyByBits::new(stored.account_meta.owner, nb_bits);
            let dict_iter = dictionary.get(&owner_partial_pubkey_by_bits);
            account_total += 1;
            if account_total > max_number_of_accounts {
                break;
            }

            log::debug!("{account_total:?}",);
            let compressed = match dict_iter {
                Some(dict_data) => {
                    accounts_with_dict += 1;
                    let instant = Instant::now();
                    let len = lz4_flex::block::compress_into_with_dict(
                        stored.data,
                        &mut compress_out_buffer,
                        dict_data,
                    )
                    .unwrap();
                    time_compression += instant.elapsed();
                    total_size_compressed += len;
                    total_size_uncompressed += stored.meta.data_len as usize;
                    compress_out_buffer[0..len].to_vec()
                }
                None => {
                    accounts_without_dict += 1;
                    let instant = Instant::now();
                    match lz4_compression {
                        Some(speed) => {
                            match lz4::block::compress(
                                stored.data,
                                Some(CompressionMode::FAST(speed)),
                                true,
                            ) {
                                Ok(data) => {
                                    time_compression += instant.elapsed();
                                    total_size_compressed += data.len();
                                    total_size_uncompressed += stored.meta.data_len as usize;
                                    data
                                }
                                Err(e) => {
                                    log::error!("error lz4 compression {e:?}");
                                    compression_errors += 1;
                                    continue;
                                }
                            }
                        }
                        None => {
                            let len =
                                lz4_flex::compress_into(stored.data, &mut compress_out_buffer)
                                    .unwrap();
                            time_compression += instant.elapsed();
                            total_size_compressed += len;
                            total_size_uncompressed += stored.meta.data_len as usize;
                            compress_out_buffer[..len].to_vec()
                        }
                    }
                }
            };

            let mut output = vec![0; stored.meta.data_len as usize];
            let decompressed = match dict_iter {
                Some(dictionary) => {
                    let instant = Instant::now();
                    match lz4_flex::block::decompress_into_with_dict(
                        &compressed,
                        &mut output,
                        dictionary,
                    ) {
                        Ok(_) => {
                            time_decompression += instant.elapsed();
                            output
                        }
                        Err(_) => {
                            decompression_errors += 1;
                            continue;
                        }
                    }
                }
                None => match lz4_compression {
                    Some(_) => {
                        let instant = Instant::now();
                        match lz4::block::decompress(&compressed, None) {
                            Ok(data) => {
                                time_decompression += instant.elapsed();
                                data
                            }
                            Err(e) => {
                                log::error!("lz4 decompression error {e:?}");
                                decompression_errors += 1;
                                continue;
                            }
                        }
                    }
                    None => {
                        let instant = Instant::now();
                        match lz4_flex::decompress_into(&compressed, &mut output) {
                            Ok(_data) => {
                                time_decompression += instant.elapsed();
                                output
                            }
                            Err(e) => {
                                log::error!(
                                    "error in decompression {e} for decompressing {} bytes",
                                    compressed.len()
                                );
                                decompression_errors += 1;
                                continue;
                            }
                        }
                    }
                },
            };
            assert_eq!(decompressed, stored.data)
        }
    }
    const ONE_MB: usize = 1024 * 1024;
    println!(
        "After lz4 compression and decompression with dictionary \n \
     {total_size_compressed} - ({} MBs) total bytes for lz compressed data, \n \
     {total_size_uncompressed} - ({} MBs) total bytes before compression, achieving\n \
     {} ({}) compression ratio, \n \
     {} ms time required to compress all data, \n \
     {} ms to decompress all data, \n \
     {accounts_with_dict} accounts used dictionary, \n \
     {accounts_without_dict} accounts did not use dictionary, \n \
     {compression_errors} compression errors, \n \
     {decompression_errors} decompression errors \n \
     {} GBps compression speed \n\
     {} GBps decompression speed \n",
        total_size_compressed / ONE_MB,
        total_size_uncompressed / ONE_MB,
        (total_size_compressed as f64 / total_size_uncompressed as f64),
        (total_size_uncompressed as f64 / total_size_compressed as f64),
        time_compression.as_millis(),
        time_decompression.as_millis(),
        (total_size_uncompressed / ONE_MB) as f64 / (time_compression.as_millis() as f64),
        (total_size_uncompressed / ONE_MB) as f64 / (time_decompression.as_millis() as f64),
    );

    Ok(())
}
