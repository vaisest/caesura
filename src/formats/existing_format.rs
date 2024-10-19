use crate::api::Torrent;
use crate::errors::AppError;
use crate::formats::SourceFormat;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Format of an existing release.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ExistingFormat {
    Flac24 = 0,
    Flac = 1,
    #[serde(rename = "320")]
    _320 = 2,
    V0 = 3,
}

impl ExistingFormat {
    #[allow(clippy::wildcard_enum_match_arm)]
    pub fn to_source(self) -> Result<SourceFormat, AppError> {
        match self {
            ExistingFormat::Flac24 => Ok(SourceFormat::Flac24),
            ExistingFormat::Flac => Ok(SourceFormat::Flac),
            _ => AppError::explained("get source format", "Format is not FLAC".to_owned()),
        }
    }
}

impl PartialOrd for ExistingFormat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExistingFormat {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = *self as isize;
        let right = *other as isize;
        left.cmp(&right)
    }
}

impl Torrent {
    #[allow(clippy::wildcard_enum_match_arm)]
    pub fn get_format(&self) -> Result<ExistingFormat, AppError> {
        match self.format.as_str() {
            "FLAC" => match self.encoding.as_str() {
                "Lossless" => Ok(ExistingFormat::Flac),
                "24bit Lossless" => Ok(ExistingFormat::Flac24),
                _ => AppError::explained(
                    "get format",
                    format!("unknown encoding: {}", self.encoding),
                ),
            },
            "MP3" => match self.encoding.as_str() {
                "320" => Ok(ExistingFormat::_320),
                "V0 (VBR)" => Ok(ExistingFormat::V0),
                _ => AppError::explained(
                    "get format",
                    format!("unknown encoding: {}", self.encoding),
                ),
            },
            _ => AppError::explained("get format", format!("unknown format: {}", self.format)),
        }
    }
}
