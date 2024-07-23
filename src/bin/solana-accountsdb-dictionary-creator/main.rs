use clap::Parser;
use solana_sdk::pubkey::Pubkey;
use std::path::PathBuf;
use std::str::FromStr;
use {
    log::info,
    solana_accountsdb_compression_dictionary_utils::{
        append_vec::AppendVec,
        append_vec_iter,
        archived::ArchiveSnapshotExtractor,
        parallel::AppendVecConsumer,
        SnapshotExtractor,
    },
    std::fs::File,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'a', long)]
    pub snapshot_archive_path: String,

    #[arg(short = 's', long, default_value_t = 2_000_000)]
    pub sample_size: usize,

    #[arg(short = 'd', long, default_value_t = 1024)]
    pub dictionary_size_per_program: usize,

    #[arg(short = 'm', long, default_value_t = 1024 * 1024 * 1024)] // 1gb
    pub max_sample_vector_length: usize,

    #[arg(short = 'o', long, default_value_t = String::from("dictionary.bin"))]
    pub out_dictionary: String,
}

#[derive(Default)]
struct Samples {
    pub samples: Vec<u8>,
    pub sizes: Vec<usize>,
    pub total_size: usize,
}

impl Samples {
    pub fn _new(data: &[u8]) -> Self {
        Self {
            samples: data.to_vec(),
            sizes: vec![data.len()],
            total_size: data.len(),
        }
    }

    pub fn add(&mut self, data: &[u8]) {
        self.sizes.push(data.len());
        self.total_size += data.len();
        self.samples.extend_from_slice(data);
    }
}

pub fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let args = Args::parse();
    println!("creator parameters are : {args:?}");

    let Args {
        snapshot_archive_path,
        sample_size,
        dictionary_size_per_program,
        out_dictionary,
        max_sample_vector_length,
    } = Args::parse();

    let archive_path = PathBuf::from_str(snapshot_archive_path.as_str()).unwrap();

    let mut loader: ArchiveSnapshotExtractor<File> =
        ArchiveSnapshotExtractor::open(&archive_path).unwrap();

    let mut samples: Samples = Samples::default();

    let mut counter = 0u64;
    for vec in loader.iter() {
        let append_vec = vec.unwrap();
        // info!("size: {:?}", append_vec.len());
        for handle in append_vec_iter(&append_vec) {
            counter += 1;
            let stored = handle.access().unwrap();
            let data_len = stored.meta.data_len as usize;
            if stored.account_meta.owner == Pubkey::default() || data_len < 1024 {
                continue;
            }
 
            if samples.sizes.len() >= sample_size
                || samples.samples.len() + data_len >= max_sample_vector_length
            {
                break;
            }
            samples.add(stored.data);
        }
    }
    println!("iterated over : {} accounts", counter);

    let dict = match zstd::dict::from_continuous(
        &samples.samples,
        &samples.sizes,
        dictionary_size_per_program,
    ) {
        Ok(v) => v,
        Err(_e) => {
            log::error!("colud not create dictioanry");
            anyhow::bail!("failed to create dictionary");
        }
    };
    let dictionary = bincode::serialize(&dict).unwrap();
    std::fs::write(out_dictionary, dictionary).unwrap();

    // println!("following programs are not included");
    // for program_id in all_program_ids {
    //     if !dictionaries.contains_key(&program_id) {
    //         let encoder = bs58::encode(program_id.to_bytes().to_vec());
    //         println!("{}", encoder.into_string());
    //     }
    // }

    Ok(())
}

struct SimpleLogConsumer {}

#[async_trait::async_trait]
impl AppendVecConsumer for SimpleLogConsumer {
    async fn on_append_vec(&mut self, append_vec: AppendVec) -> anyhow::Result<()> {
        info!("size: {:?}", append_vec.len());
        info!("slot: {:?}", append_vec.slot());
        for handle in append_vec_iter(&append_vec) {
            let stored = handle.access().unwrap();
            info!(
                "account {:?}: {} at slot {}",
                stored.meta.pubkey,
                stored.account_meta.lamports,
                append_vec.slot()
            );
        }
        Ok(())
    }
}
