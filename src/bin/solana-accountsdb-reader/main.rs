use std::path::PathBuf;
use std::str::FromStr;
use log::warn;
use {
    log::info,
    reqwest::blocking::Response,
    solana_accountsdb_reader::{
        append_vec::AppendVec,
        append_vec_iter,
        archived::ArchiveSnapshotExtractor,
        parallel::{par_iter_append_vecs, AppendVecConsumer},
        unpacked::UnpackedSnapshotExtractor,
        AppendVecIterator, ReadProgressTracking, SnapshotError, SnapshotExtractor, SnapshotResult,
    },
    std::{
        fs::File,
        io::{IoSliceMut, Read},
        path::Path,
        sync::Arc,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let archive_path = PathBuf::from_str("/Users/stefan/mango/projects/snapshot-reader/snapshot-78017-6vfFEs6YnFZPfPRBPnjqgMN8UmE5jnpBGscKXsPCtdV7.tar.zst").unwrap();

    let mut loader: ArchiveSnapshotExtractor<File> = ArchiveSnapshotExtractor::open(&archive_path).unwrap();

    // for vec in loader.iter() {
    //     let append_vec =  vec.unwrap();
    //     info!("size: {:?}", append_vec.len());
    //     for handle in append_vec_iter(&append_vec) {
    //         let stored = handle.access().unwrap();
    //         info!("account {:?}: {}", stored.meta.pubkey, stored.account_meta.lamports);
    //     }
    // }

    par_iter_append_vecs(
        loader.iter(),
        || SimpleLogConsumer {
        },
        4,
    )
        .await?;

    Ok(())
}

pub enum SupportedLoader {
    Unpacked(UnpackedSnapshotExtractor),
    ArchiveFile(ArchiveSnapshotExtractor<File>),
    ArchiveDownload(ArchiveSnapshotExtractor<Response>),
}

impl SupportedLoader {
    fn new(source: &str, progress_tracking: Box<dyn ReadProgressTracking>) -> anyhow::Result<Self> {
        if source.starts_with("http://") || source.starts_with("https://") {
            Self::new_download(source)
        } else {
            Self::new_file(source.as_ref(), progress_tracking).map_err(Into::into)
        }
    }

    fn new_download(url: &str) -> anyhow::Result<Self> {
        let resp = reqwest::blocking::get(url)?;
        let loader = ArchiveSnapshotExtractor::from_reader(resp)?;
        info!("Streaming snapshot from HTTP");
        Ok(Self::ArchiveDownload(loader))
    }

    fn new_file(
        path: &Path,
        progress_tracking: Box<dyn ReadProgressTracking>,
    ) -> solana_accountsdb_reader::SnapshotResult<Self> {
        Ok(if path.is_dir() {
            info!("Reading unpacked snapshot");
            Self::Unpacked(UnpackedSnapshotExtractor::open(path, progress_tracking)?)
        } else {
            info!("Reading snapshot archive");
            Self::ArchiveFile(ArchiveSnapshotExtractor::open(path)?)
        })
    }
}

impl SnapshotExtractor for SupportedLoader {
    fn iter(&mut self) -> AppendVecIterator<'_> {
        match self {
            SupportedLoader::Unpacked(loader) => Box::new(loader.iter()),
            SupportedLoader::ArchiveFile(loader) => Box::new(loader.iter()),
            SupportedLoader::ArchiveDownload(loader) => Box::new(loader.iter()),
        }
    }
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
            info!("account {:?}: {}", stored.meta.pubkey, stored.account_meta.lamports);
        }
        Ok(())
    }
}
