use crate::errors::AppError;
use crate::fs::DirectoryReader;
use crate::options::{CacheOptions, Options, QueueOptions, SharedOptions};
use crate::queue::{Queue, QueueStatus};
use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{debug, info, trace};
use std::path::PathBuf;

/// Add a directory of `.torrent` files to the queue
#[injectable]
pub struct QueueAddCommand {
    shared_options: Ref<SharedOptions>,
    cache_options: Ref<CacheOptions>,
    queue_options: Ref<QueueOptions>,
    queue: RefMut<Queue>,
}

impl QueueAddCommand {
    pub async fn execute_cli(&mut self) -> Result<bool, AppError> {
        if !self.shared_options.validate()
            || !self.cache_options.validate()
            || !self.queue_options.validate()
        {
            return Ok(false);
        }
        let torrent_dir = self
            .queue_options
            .torrents
            .clone()
            .expect("source should be set");
        let status = self.execute(torrent_dir).await?;
        info!("{} {} items to the queue", "Added".bold(), status.added);
        trace!(
            "{} {} items already in the queue",
            "Excluded".bold(),
            status.excluded
        );
        trace!("{} {} items in the queue", "Total".bold(), status.total);
        Ok(true)
    }

    pub async fn execute(&mut self, torrent_dir: PathBuf) -> Result<QueueStatus, AppError> {
        if !torrent_dir.is_dir() {
            return AppError::explained("get torrent files", "path is not a directory".to_owned());
        }
        trace!("Reading torrent directory: {}", torrent_dir.display());
        let paths = DirectoryReader::new()
            .with_extension("torrent")
            .with_max_depth(0)
            .read(&torrent_dir)
            .or_else(|e| AppError::io(e, "read torrent directory"))?;
        let found = paths.len();
        debug!("Found {} torrent files", found);
        if found > 250 {
            debug!("This may take a while");
        }
        let mut queue = self.queue.write().expect("queue should be writeable");
        queue.load()?;
        let added = queue.insert_new_torrent_files(paths).await;
        queue.save()?;
        Ok(QueueStatus {
            success: true,
            added,
            excluded: found - added,
            total: queue.len(),
        })
    }
}
