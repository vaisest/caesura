use std::cmp::Ordering;
use crate::formats::ExistingFormat;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use crate::formats::TargetFormat::{_320, Flac, V0};

/// Format to transcode to.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum TargetFormat {
    Flac,
    #[serde(rename = "320")]
    _320,
    V0,
}

impl TargetFormat {

    #[must_use]
    pub fn get_name(&self) -> &str {
        match self {
            Flac => "FLAC",
            _320 => "320",
            V0 => "V0",
        }
    }

    #[must_use]
    pub fn to_existing(self) -> ExistingFormat {
        match self {
            Flac => ExistingFormat::Flac,
            _320 => ExistingFormat::_320,
            V0 => ExistingFormat::V0,
        }
    }

    #[must_use]
    pub fn get_file_extension(self) -> String {
        match self {
            Flac => "flac".to_owned(),
            _320 => "mp3".to_owned(),
            V0 => "mp3".to_owned(),
        }
    }
}

impl PartialOrd for TargetFormat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_name().len().partial_cmp(&other.get_name().len())
    }
}

impl Ord for TargetFormat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_name().len().cmp(&other.get_name().len())
    }
}
