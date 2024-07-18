use std::{fs::File, path::PathBuf, str::FromStr, time::{Duration, Instant}};

use clap::Parser;
use solana_accountsdb_compression_dictionary_utils::{append_vec_iter, archived::ArchiveSnapshotExtractor, partial_pubkey::DictionaryMap, SnapshotExtractor};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'a', long)]
    pub snapshot_archive_path: String,

    #[arg(short = 'd', long, default_value_t = String::from("dictionary.bin"))]
    pub dictionary: String,
}


pub fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let Args { snapshot_archive_path,  dictionary} = Args::parse();

    let archive_path = PathBuf::from_str(snapshot_archive_path.as_str()).unwrap();

    let mut loader: ArchiveSnapshotExtractor<File> = ArchiveSnapshotExtractor::open(&archive_path).unwrap();

    // loading dictionary
    let dictionary_binary = std::fs::read(dictionary).unwrap();
    let dictionary = bincode::deserialize::<DictionaryMap>(&dictionary_binary).unwrap();
    drop(dictionary_binary);

    let mut total_size: usize = 0;
    let mut accounts_with_dict:usize = 0;
    let mut accounts_without_dict:usize = 0;
    let mut compression_errors: usize = 0;
    let mut decompression_errors: usize = 0;
    let  mut time_compression: Duration = Duration::from_micros(0);
    let mut time_decompression: Duration = Duration::from_micros(0);

    let mut buf = vec![0; 64 * 1024 * 1024]; // 64MB;
    for vec in loader.iter() {
        let append_vec =  vec.unwrap();
        // info!("size: {:?}", append_vec.len());
        for handle in append_vec_iter(&append_vec) {
            let stored = handle.access().unwrap();
            if stored.meta.data_len < 8 {
                total_size+=stored.meta.data_len as usize;
                continue;
            }
            let dict_iter = dictionary.get(&stored.account_meta.owner.into());
            let compressed = match dict_iter {
                Some(dict_data) => {
                    accounts_with_dict += 1;
                    let instant = Instant::now();
                    match lz4_flex::block::compress_into_with_dict(stored.data, buf.as_mut_slice(), dict_data) {
                        Ok(size) => {
                            time_compression += instant.elapsed();
                            total_size += size;
                            buf[..size].to_vec()
                        },
                        Err(_) => {
                            compression_errors += 1;
                            continue;
                        },
                    }
                },
                None => {
                    accounts_without_dict += 1;
                    let instant = Instant::now();
                    let data = lz4_flex::compress(stored.data);
                    time_compression += instant.elapsed();
                    total_size += data.len();
                    data
                },
            };

            let decompressed = match dict_iter {
                Some(dictionary) => {
                    let instant = Instant::now();
                    match lz4_flex::block::decompress_into_with_dict(&compressed, buf.as_mut_slice(), dictionary) {
                        Ok(size) => {
                            time_decompression += instant.elapsed();
                            buf[..size].to_vec()
                        },
                        Err(_) => {
                            decompression_errors+=1;
                            continue;
                        },
                    }
                     
                },
                None => {
                    let instant = Instant::now();
                    let data = lz4_flex::decompress(&compressed, 0).unwrap();
                    time_decompression += instant.elapsed();
                    data
                },
            };
            assert_eq!(decompressed, stored.data)
        }
    }
    log::info!("After lz4 compression and decompression with dictionary \
     {total_size} total bytes for lz compressed data, \
     {} ms time required to compress all data, \
     {} ms to decompress all data, \
     {accounts_with_dict} accounts used dictionary, \
     {accounts_without_dict} accounts did not use dictionary, \
     {compression_errors} compression errors,
     {decompression_errors} decompression errors", 
      time_compression.as_millis(), time_decompression.as_millis());

    Ok(())
}