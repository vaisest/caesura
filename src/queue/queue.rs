use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use colored::Colorize;
use di::{inject, injectable, Ref};
use log::{error, trace};

use crate::errors::AppError;
use crate::imdl::ImdlCommand;
use crate::options::QueueOptions;
use crate::queue::QueueItem;
use crate::verify::VerifyStatus;

#[injectable]
pub struct Queue {
    /// Path to the queue file
    path: PathBuf,

    /// Items in the queue
    /// Key is torrent hash
    items: BTreeMap<String, QueueItem>,
}

#[allow(dead_code)]
impl Queue {
    /// Create a new [`Queue`]
    #[allow(dead_code)]
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            path,
            items: BTreeMap::new(),
        }
    }

    /// DI constructor for [`Queue`]
    #[inject]
    pub fn from_options(options: Ref<QueueOptions>) -> Self {
        let path = options.queue.clone().expect("queue path should be set");
        Self {
            path,
            items: BTreeMap::new(),
        }
    }

    /// Get an item from the queue
    pub fn get(&self, hash: &str) -> Option<&QueueItem> {
        self.items.get(hash)
    }

    /// Get the keys of the items that have not been processed.
    ///
    /// If `skip_upload` is true, only items that have not been uploaded will be returned.
    /// Otherwise, items that have not been transcoded or uploaded will be returned.
    ///
    /// Items are sorted by name
    pub fn get_unprocessed(&mut self, indexer: String, skip_upload: bool) -> Vec<String> {
        let mut items: Vec<&QueueItem> = self
            .items
            .values()
            .filter(|x| {
                x.skipped.is_none()
                    && x.uploaded.is_none()
                    && (!skip_upload || x.transcoded.is_none())
                    && x.indexer == indexer
            })
            .collect();
        items.sort_by_key(|x| x.name.clone());
        items.iter().map(|x| x.hash.clone()).collect()
    }

    /// Set the skipped reason for an item
    pub fn set_skipped(&mut self, hash: String, reason: String) {
        self.items
            .entry(hash)
            .and_modify(|x| x.skipped = Some(reason));
    }

    /// Set the failed reason for an item
    pub fn set_failed(&mut self, hash: String, reason: String) {
        self.items
            .entry(hash)
            .and_modify(|x| x.failed = Some(reason));
    }

    /// Set the verified status for an item
    pub fn set_verified(&mut self, hash: String, status: VerifyStatus) {
        self.items
            .entry(hash)
            .and_modify(|x| x.verified = Some(status));
    }

    /// Set the transcoded status for an item
    pub fn set_transcoded(&mut self, hash: String) {
        self.items
            .entry(hash)
            .and_modify(|x| x.transcoded = Some(true));
    }

    /// Set the uploaded status for an item
    pub fn set_uploaded(&mut self, hash: String) {
        self.items
            .entry(hash)
            .and_modify(|x| x.uploaded = Some(true));
    }

    /// Insert an item into the queue
    pub fn insert(&mut self, item: QueueItem) {
        self.items.insert(item.hash.clone(), item);
    }

    /// Insert torrent files into the queue if they are not already present
    /// Returns the number of items added
    pub async fn insert_new_torrent_files(&mut self, paths: Vec<PathBuf>) -> usize {
        let existing: Vec<PathBuf> = self.items.values().map(|x| x.path.clone()).collect();
        let mut added = 0;
        for path in paths {
            if self.insert_new_torrent_file(&existing, path).await {
                added += 1;
            }
        }
        added
    }

    /// Insert torrent files into the queue if they are not already present
    /// Returns `true` if the item was inserted
    /// Returns `false` if an item in the queue already had the same path or hash.
    async fn insert_new_torrent_file(&mut self, existing: &[PathBuf], path: PathBuf) -> bool {
        if existing.contains(&path) {
            return false;
        }
        trace!("Reading torrent: {path:?}");
        let torrent = match ImdlCommand::show(&path).await {
            Ok(torrent) => torrent,
            Err(error) => {
                error!("Failed to read torrent: {path:?}\n{error}");
                return false;
            }
        };
        let item = QueueItem::from_torrent(path, torrent);
        if self.items.contains_key(&item.hash) {
            return false;
        }
        self.items.insert(item.hash.clone(), item);
        true
    }

    /// Save the queue to a YAML serialized file
    ///
    /// Items are sorted by name if `sort` is true
    pub fn save(&self) -> Result<(), AppError> {
        let path = self.path.clone();
        if !path.exists() || !path.is_file() {
            return AppError::explained("write queue", "queue file does not exist".to_owned());
        }
        trace!("{} queue file: {:?}", "Writing".bold(), path);
        let file = File::create(path).or_else(|e| AppError::io(e, "open queue"))?;
        let mut writer = BufWriter::new(file);
        serde_yaml::to_writer(&mut writer, &self.items)
            .or_else(|e| AppError::yaml(e, "serialize queue"))?;
        writer
            .flush()
            .or_else(|e| AppError::external("flush queue", "BufWriter", format!("{e}")))?;
        Ok(())
    }

    /// Load a queue from a path
    pub fn load(&mut self) -> Result<(), AppError> {
        let path = self.path.clone();
        if !path.exists() || !path.is_file() {
            return AppError::explained("load queue", "queue file does not exist".to_owned());
        }
        trace!("{} queue from: {path:?}", "Loading".bold());
        let file = File::open(path.clone()).or_else(|e| AppError::io(e, "open queue file"))?;
        if file.metadata().expect("file should have metadata").len() == 0 {
            trace!("Queue file is empty: {path:?}");
        } else {
            let reader = BufReader::new(file);
            let items: HashMap<String, QueueItem> = serde_yaml::from_reader(reader)
                .or_else(|e| AppError::yaml(e, "deserialize queue file"))?;
            let len = items.len();
            self.items.extend(items);
            trace!("{} {len} items", "Loaded".bold());
        }
        Ok(())
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
