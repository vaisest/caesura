use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use colored::Colorize;
use log::trace;

use crate::batch::BatchItem;
use crate::errors::AppError;

pub struct BatchCache {
    pub path: Option<PathBuf>,
    pub items: HashMap<PathBuf, BatchItem>,
}

impl BatchCache {
    pub fn get_queue(&mut self) -> Vec<BatchItem> {
        let mut queue: Vec<BatchItem> = self
            .items
            .values()
            .filter(|x| x.skipped.is_none() && !x.uploaded)
            .map(Clone::clone)
            .collect();
        queue.sort_by_key(|x| x.path.to_string_lossy().to_string());
        queue
    }

    pub fn update<F>(&mut self, path: &Path, function: F)
    where
        F: FnOnce(&mut BatchItem),
    {
        let key = PathBuf::from(path);
        self.items.entry(key).and_modify(function);
    }

    pub fn save(&self) -> Result<(), AppError> {
        if let Some(path) = &self.path {
            trace!("{} cache file: {path:?}", "Writing".bold());
            let file = File::create(path).or_else(|e| AppError::io(e, "open batch cache"))?;
            let mut writer = BufWriter::new(file);
            let items: Vec<&BatchItem> = self.items.values().collect();
            serde_json::to_writer_pretty(&mut writer, &items)
                .or_else(|e| AppError::deserialization(e, "serialize batch cache"))?;
            writer
                .flush()
                .or_else(|e| AppError::external("flush batch cache", "BufWriter", Box::new(e)))?;
        }
        Ok(())
    }
}
