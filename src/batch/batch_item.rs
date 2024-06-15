use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct BatchItem {
    pub path: PathBuf,
    pub skipped: Option<String>,
    pub failed: Option<String>,
    pub uploaded: bool,
}

impl BatchItem {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            skipped: None,
            uploaded: false,
            failed: None,
        }
    }

    pub fn set_skipped(&mut self, reason: String) {
        self.skipped = Some(reason);
    }

    pub fn set_failed(&mut self, reason: String) {
        self.failed = Some(reason);
    }

    pub fn set_uploaded(&mut self) {
        self.uploaded = true;
    }
}

impl Display for BatchItem {
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
