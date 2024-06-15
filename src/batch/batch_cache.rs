use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use colored::Colorize;
use log::trace;

use crate::batch::BatchItem;
use crate::errors::AppError;
use crate::fs::DirectoryReader;

pub struct BatchCache {
    items: HashMap<PathBuf, BatchItem>,
}

impl BatchCache {
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn get_queue(&mut self) -> Vec<BatchItem> {
        self.items
            .values()
            .filter(|x| x.skipped.is_none() && !x.uploaded)
            .map(Clone::clone)
            .collect()
    }

    pub fn update<F>(&mut self, path: &Path, function: F)
    where
        F: FnOnce(&mut BatchItem),
    {
        let key = PathBuf::from(path);
        self.items.entry(key).and_modify(function);
    }

    fn from_vec(items: Vec<BatchItem>) -> Self {
        let mut map = HashMap::new();
        for item in items {
            map.insert(item.path.clone(), item);
        }
        Self { items: map }
    }

    pub fn from_file(path: &Path) -> Result<BatchCache, AppError> {
        let file = File::open(path).or_else(|e| AppError::io(e, "open batch cache"))?;
        if file.metadata().expect("file should have metadata").len() == 0 {
            trace!("Cache file is empty: {path:?}");
            Ok(BatchCache::new())
        } else {
            trace!("{} cache file: {path:?}", "Reading".bold());
            let reader = BufReader::new(file);
            let items = serde_json::from_reader(reader)
                .or_else(|e| AppError::deserialization(e, "deserialize batch cache"))?;
            Ok(BatchCache::from_vec(items))
        }
    }

    pub fn write(self, path: &Path) -> Result<(), AppError> {
        trace!("{} cache file: {path:?}", "Writing".bold());
        let file = File::create(path).or_else(|e| AppError::io(e, "open batch cache"))?;
        let mut writer = BufWriter::new(file);
        let items: Vec<BatchItem> = self
            .items
            .into_values()
            .collect();
        serde_json::to_writer(&mut writer, &items)
            .or_else(|e| AppError::deserialization(e, "serialize batch cache"))?;
        writer
            .flush()
            .or_else(|e| AppError::external("flush batch cache", "BufWriter", Box::new(e)))?;
        Ok(())
    }

    pub fn load_from(&mut self, source: String) -> Result<(), AppError> {
        let source = PathBuf::from(source);
        if !source.is_dir() {
            return AppError::explained(
                "get source by directory",
                "path is not a directory".to_owned(),
            );
        }
        let paths = DirectoryReader::new()
            .with_extension("torrent")
            .with_max_depth(0)
            .read(&source)
            .or_else(|e| AppError::io(e, "read source directory"))?;
        for path in paths {
            if !self.items.contains_key(&path) {
                self.items.insert(path.clone(), BatchItem::new(path));
            }
        }
        Ok(())
    }
}
