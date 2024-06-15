use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use colored::Colorize;
use di::{injectable, Ref};
use log::trace;

use crate::batch::{BatchCache, BatchItem};
use crate::errors::AppError;
use crate::fs::DirectoryReader;
use crate::options::{BatchOptions, Options, SharedOptions};

#[injectable]
pub struct BatchCacheFactory {
    shared_options: Ref<SharedOptions>,
    batch_options: Ref<BatchOptions>,
}

impl BatchCacheFactory {
    pub fn create(&mut self) -> Result<BatchCache, AppError> {
        let cache_path = self.batch_options.cache.clone();
        let mut cache: HashMap<PathBuf, BatchItem> = HashMap::new();
        if let Some(path) = &cache_path {
            trace!("{} cache file: {path:?}", "Reading".bold());
            insert_items_from_file(&mut cache, &path.clone())?;
        }
        let directory = self.shared_options.get_value(|x| x.source.clone());
        insert_items_from_directory(&mut cache, &PathBuf::from(directory))?;
        Ok(BatchCache {
            path: cache_path,
            items: cache,
        })
    }
}

fn insert_items_from_file(
    cache: &mut HashMap<PathBuf, BatchItem>,
    path: &Path,
) -> Result<(), AppError> {
    if !path.exists() || !path.is_file() {
        return Ok(());
    }
    let file = File::open(path).or_else(|e| AppError::io(e, "open batch cache"))?;
    if file.metadata().expect("file should have metadata").len() == 0 {
        trace!("Cache file is empty: {path:?}");
        Ok(())
    } else {
        let reader = BufReader::new(file);
        let items = serde_json::from_reader(reader)
            .or_else(|e| AppError::deserialization(e, "deserialize batch cache"))?;
        insert_vec(cache, items);
        Ok(())
    }
}

fn insert_items_from_directory(
    cache: &mut HashMap<PathBuf, BatchItem>,
    directory: &Path,
) -> Result<(), AppError> {
    if !directory.is_dir() {
        return AppError::explained(
            "get source by directory",
            "path is not a directory".to_owned(),
        );
    }
    let paths = DirectoryReader::new()
        .with_extension("torrent")
        .with_max_depth(0)
        .read(directory)
        .or_else(|e| AppError::io(e, "read source directory"))?;
    for path in paths {
        insert_new(cache, path);
    }
    Ok(())
}

fn insert_new(cache: &mut HashMap<PathBuf, BatchItem>, path: PathBuf) {
    if !cache.contains_key(&path) {
        cache.insert(path.clone(), BatchItem::new(path));
    }
}

fn insert_vec(cache: &mut HashMap<PathBuf, BatchItem>, items: Vec<BatchItem>) {
    for item in items {
        if !cache.contains_key(&item.path) {
            cache.insert(item.path.clone(), item);
        }
    }
}
