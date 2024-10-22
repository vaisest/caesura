use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use colored::Colorize;
use di::{inject, injectable, Ref};
use log::{error, trace};

use crate::errors::AppError;
use crate::imdl::ImdlCommand;
use crate::options::CacheOptions;
use crate::queue::QueueItem;
use crate::spectrogram::SpectrogramStatus;
use crate::transcode::TranscodeStatus;
use crate::upload::UploadStatus;
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
    pub fn from_options(options: Ref<CacheOptions>) -> Self {
        let path = options.cache.clone().expect("queue path should be set");
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
    /// Items are filtered to ensure they have:
    /// - the correct indexer
    /// - not been verified, unless `transcode_enabled` is true
    /// - not been transcoded, unless `upload_enabled` is true
    /// - not been verified OR have been and `verified` is true
    /// - not been transcoded OR have been and `success` is true
    /// - not been uploaded
    ///
    /// Items are sorted by name
    pub fn get_unprocessed(
        &mut self,
        indexer: String,
        transcode_enabled: bool,
        upload_enabled: bool,
    ) -> Vec<String> {
        let mut items: Vec<&QueueItem> = self
            .items
            .values()
            .filter(|item| {
                item.indexer == indexer
                    && exclude_verified_if_transcode_disabled(item, transcode_enabled)
                    && exclude_transcoded_if_upload_disabled(item, upload_enabled)
                    && exclude_verify_failures(item)
                    && exclude_transcode_failures(item)
                    && item.upload.is_none()
            })
            .collect();
        items.sort_by_key(|x| x.name.clone());
        items.iter().map(|x| x.hash.clone()).collect()
    }

    /// Set the verify status of an item
    pub fn set_verify(&mut self, hash: String, status: VerifyStatus) {
        self.items
            .entry(hash)
            .and_modify(|x| x.verify = Some(status));
    }

    /// Set the spectrogram status of an item
    pub fn set_spectrogram(&mut self, hash: String, status: SpectrogramStatus) {
        self.items
            .entry(hash)
            .and_modify(|x| x.spectrogram = Some(status));
    }

    /// Set the transcode status of an item
    pub fn set_transcode(&mut self, hash: String, status: TranscodeStatus) {
        self.items
            .entry(hash)
            .and_modify(|x| x.transcode = Some(status));
    }

    /// Set the upload status of an item
    pub fn set_upload(&mut self, hash: String, status: UploadStatus) {
        self.items
            .entry(hash)
            .and_modify(|x| x.upload = Some(status));
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
        let torrent = match ImdlCommand::show(&path).await {
            Ok(torrent) => torrent,
            Err(error) => {
                error!("Failed to read torrent: {}\n{error}", path.display());
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
        trace!("{} queue file: {}", "Writing".bold(), path.display());
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
        trace!("{} queue from: {}", "Loading".bold(), path.display());
        let file = File::open(path.clone()).or_else(|e| AppError::io(e, "open queue file"))?;
        if file.metadata().expect("file should have metadata").len() == 0 {
            trace!("Queue file is empty: {}", path.display());
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

fn exclude_verify_failures(item: &QueueItem) -> bool {
    if let Some(verify) = &item.verify {
        verify.verified
    } else {
        true
    }
}

fn exclude_transcode_failures(item: &QueueItem) -> bool {
    if let Some(transcode) = &item.transcode {
        transcode.success
    } else {
        true
    }
}

fn exclude_verified_if_transcode_disabled(item: &QueueItem, transcode_enabled: bool) -> bool {
    transcode_enabled || item.verify.is_none()
}

fn exclude_transcoded_if_upload_disabled(x: &QueueItem, upload_enabled: bool) -> bool {
    upload_enabled || x.transcode.is_none()
}
