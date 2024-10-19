use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use colored::Colorize;
use log::trace;

use crate::errors::AppError;
use crate::queue::QueueItem;
use crate::verify::VerifyStatus;

pub struct Queue {
    /// Path to the queue file
    path: PathBuf,

    /// Items in the queue
    /// Key is torrent hash
    items: BTreeMap<String, QueueItem>,
}

impl Queue {
    /// Create a new [`Queue`]
    pub fn new(path: PathBuf) -> Self {
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

    /// Save the queue to a YAML serialized file
    ///
    /// Items are sorted by name if `sort` is true
    pub fn save(&self) -> Result<(), AppError> {
        trace!("{} queue file: {:?}", "Writing".bold(), self.path);
        let file = File::create(self.path.clone()).or_else(|e| AppError::io(e, "open queue"))?;
        let mut writer = BufWriter::new(file);
        serde_yaml::to_writer(&mut writer, &self.items)
            .or_else(|e| AppError::yaml(e, "serialize queue"))?;
        writer
            .flush()
            .or_else(|e| AppError::external("flush queue", "BufWriter", format!("{e}")))?;
        Ok(())
    }

    /// Load a queue from a path
    pub fn load(&mut self, path: PathBuf) -> Result<(), AppError> {
        if !path.exists() || !path.is_file() {
            return AppError::explained("load queue", "queue file does not exist".to_owned());
        }
        let file = File::open(path.clone()).or_else(|e| AppError::io(e, "open queue file"))?;
        if file.metadata().expect("file should have metadata").len() == 0 {
            trace!("Queue file is empty: {path:?}");
        } else {
            let reader = BufReader::new(file);
            let items: HashMap<String, QueueItem> = serde_yaml::from_reader(reader)
                .or_else(|e| AppError::yaml(e, "deserialize queue file"))?;
            self.items.extend(items);
        }
        Ok(())
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
