use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct BatchCacheItem {
    pub path: PathBuf,
    pub skipped: Option<String>,
    pub failed: Option<String>,
    #[serde(default)]
    pub transcoded: bool,
    pub uploaded: bool,
}

impl BatchCacheItem {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            skipped: None,
            uploaded: false,
            transcoded: false,
            failed: None,
        }
    }

    pub fn set_skipped(&mut self, reason: String) {
        self.skipped = Some(reason);
    }

    pub fn set_failed(&mut self, reason: String) {
        self.failed = Some(reason);
    }

    pub fn set_transcoded(&mut self) {
        self.transcoded = true;
    }

    pub fn set_uploaded(&mut self) {
        self.uploaded = true;
    }
}

impl Display for BatchCacheItem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let file_name = self
            .path
            .file_name()
            .expect("should have a file name")
            .to_string_lossy()
            .to_string();
        write!(formatter, "{file_name}")
    }
}
