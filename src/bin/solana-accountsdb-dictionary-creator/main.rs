use std::collections::HashMap;
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
        partial_pubkey::{DictionaryMap, PartialPubkey}
    },
    std::fs::File,
};
use clap::Parser;
use itertools::Itertools;
use solana_sdk::pubkey::Pubkey;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'a', long)]
    pub snapshot_archive_path: String,

    #[arg(short = 's', long, default_value_t = 100_000)]
    pub sample_size: usize,

    #[arg(short = 'd', long, default_value_t = 64 * 1024)]
    pub dictionary_size_per_program: usize,

    #[arg(short = 'o', long, default_value_t = String::from("dictionary.bin"))]
    pub out_dictionary: String,
}

struct Samples {
    pub samples: Vec<u8>,
    pub sizes: Vec<usize>,
    pub total_size: usize,
}

impl Samples {
    pub fn new(data: &[u8]) -> Self {
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

    let Args { snapshot_archive_path,  sample_size, dictionary_size_per_program, out_dictionary} = Args::parse();

    let archive_path = PathBuf::from_str(snapshot_archive_path.as_str()).unwrap();

    let mut loader: ArchiveSnapshotExtractor<File> = ArchiveSnapshotExtractor::open(&archive_path).unwrap();

    let mut samples: HashMap<PartialPubkey<4>, Samples> = HashMap::new();

    let mut counter = 0u64;
    for vec in loader.iter() {
        let append_vec =  vec.unwrap();
        // info!("size: {:?}", append_vec.len());
        for handle in append_vec_iter(&append_vec) {
            counter+=1;
            let stored = handle.access().unwrap();
            if stored.account_meta.owner == Pubkey::default() || stored.meta.data_len < 8 {
                continue;
            }
            
            let data = stored.data;
            let key = stored.account_meta.owner.into();
            match samples.entry(key) {
                std::collections::hash_map::Entry::Occupied(mut occ) => {
                    let val =  occ.get_mut();
                    if val.sizes.len() >= sample_size {
                        continue;
                    }
                    val.add(data);
                },
                std::collections::hash_map::Entry::Vacant(vac) => {
                    vac.insert(Samples::new(data));
                },
            };
        }
    }
    println!("iterated over : {} accounts", counter);
    let all_program_ids = samples.iter().map(|x| *x.0).collect_vec();

    let mut dictionaries = DictionaryMap::new();
    for (key, ite_sample) in samples.drain() {
        let dict = match zstd::dict::from_continuous(&ite_sample.samples, &ite_sample.sizes, dictionary_size_per_program) {
            Ok(v) => v,
            Err(e) => {
                println!("error {}, ite_sample: {}, number of samples: {}", e, ite_sample.samples.len(), ite_sample.sizes.len());
                continue;
            },
        };
        dictionaries.insert(key, dict);
    }
    println!("program ids in dictionaries : {}/{}", dictionaries.len(), all_program_ids.len());
    let dictionary = bincode::serialize(&dictionaries).unwrap();
    std::fs::write(out_dictionary, dictionary).unwrap();

    println!("following programs are not included");
    for program_id in all_program_ids {
        if !dictionaries.contains_key(&program_id) {
            let encoder = bs58::encode(program_id.to_bytes().to_vec());
            println!("{}", encoder.into_string());
        }
    }

    Ok(())
}

struct SimpleLogConsumer {
}

#[async_trait::async_trait]
impl AppendVecConsumer for SimpleLogConsumer {
    async fn on_append_vec(&mut self, append_vec: AppendVec) -> anyhow::Result<()> {
        info!("size: {:?}", append_vec.len());
        info!("slot: {:?}", append_vec.slot());
        for handle in append_vec_iter(&append_vec) {
            let stored = handle.access().unwrap();
            info!("account {:?}: {} at slot {}", stored.meta.pubkey, stored.account_meta.lamports, append_vec.slot());
        }
        Ok(())
    }
}
