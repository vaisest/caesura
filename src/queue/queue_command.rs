use crate::errors::AppError;
use crate::fs::DirectoryReader;
use crate::options::{Options, QueueOptions, SharedOptions};
use crate::queue::{Queue, QueueStatus};
use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{info, trace};
use std::path::PathBuf;

/// Add a directory of `.torrent` files to the queue
#[injectable]
pub struct QueueCommand {
    shared_options: Ref<SharedOptions>,
    queue_options: Ref<QueueOptions>,
    queue: RefMut<Queue>,
}

impl QueueCommand {
    pub async fn execute_cli(&mut self) -> Result<bool, AppError> {
        if !self.shared_options.validate() || !self.queue_options.validate() {
            return Ok(false);
        }
        let source = self
            .shared_options
            .source
            .clone()
            .expect("source should be set");
        let source_dir = PathBuf::from(source);
        let status = self.execute(source_dir).await?;
        info!("{} {} items to the queue", "Added".bold(), status.added);
        trace!(
            "{} {} items already in the queue",
            "Excluded".bold(),
            status.excluded
        );
        trace!("{} {} items in the queue", "Total".bold(), status.total);
        Ok(true)
    }

    pub async fn execute(&mut self, source_dir: PathBuf) -> Result<QueueStatus, AppError> {
        if !source_dir.is_dir() {
            return AppError::explained(
                "get source by directory",
                "path is not a directory".to_owned(),
            );
        }
        trace!("Reading source directory: {source_dir:?}");
        let paths = DirectoryReader::new()
            .with_extension("torrent")
            .with_max_depth(0)
            .read(&source_dir)
            .or_else(|e| AppError::io(e, "read source directory"))?;
        let found = paths.len();
        trace!("Found {} torrent files", found);
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
