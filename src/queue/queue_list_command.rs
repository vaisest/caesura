use crate::errors::AppError;
use crate::options::{BatchOptions, CacheOptions, Options, SharedOptions};
use crate::queue::Queue;
use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{debug, error, info};

/// List the sources in the queue
#[injectable]
pub struct QueueListCommand {
    shared_options: Ref<SharedOptions>,
    cache_options: Ref<CacheOptions>,
    batch_options: Ref<BatchOptions>,
    queue: RefMut<Queue>,
}

impl QueueListCommand {
    pub fn execute_cli(&mut self) -> Result<bool, AppError> {
        if !self.shared_options.validate()
            || !self.cache_options.validate()
            || !self.batch_options.validate()
        {
            return Ok(false);
        }
        let mut queue = self.queue.write().expect("Queue should be writeable");
        queue.load()?;
        let transcode_enabled = self
            .batch_options
            .transcode
            .expect("transcode should be set");
        let upload_enabled = self.batch_options.upload.expect("upload should be set");
        let indexer = self
            .shared_options
            .indexer
            .clone()
            .expect("indexer should be set");
        let items = queue.get_unprocessed(indexer.clone(), transcode_enabled, upload_enabled);
        if items.is_empty() {
            info!(
                "{} items in the queue for {}",
                "No".bold(),
                indexer.to_uppercase()
            );
            info!("{} the `queue` command to add items", "Use".bold());
            return Ok(true);
        }
        info!(
            "{} {} unprocessed sources in the queue for {}",
            "Found".bold(),
            items.len(),
            indexer.to_uppercase()
        );
        let mut count = 1;
        for hash in items {
            let Some(item) = queue.get(&hash) else {
                error!("{} to retrieve {hash} from the queue", "Failed".bold());
                continue;
            };
            info!("{}: {item}", count.to_string().bold());
            debug!("{}", item.path.display());
            debug!("{hash}");
            if let Some(id) = item.id {
                debug!("{id}");
            }
            count += 1;
        }
        Ok(true)
    }
}
