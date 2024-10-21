use crate::errors::AppError;
use crate::formats::TargetFormat;
use crate::queue::TimeStamp;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct TranscodeStatus {
    /// Did the transcode command succeed?
    pub success: bool,
    /// Transcode formats
    pub formats: Option<Vec<TranscodeFormatStatus>>,
    /// Additional files
    #[allow(dead_code)]
    pub additional: Option<Vec<AdditionalStatus>>,
    /// Time the transcode completed
    pub completed: TimeStamp,
    /// Error message if the transcode failed
    pub error: Option<AppError>,
}

#[derive(Deserialize, Serialize)]
pub struct TranscodeFormatStatus {
    /// Did the transcode command succeed?
    pub format: TargetFormat,
    /// Path to the transcode directory
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize)]
pub struct AdditionalStatus {
    /// Relative path of the additional file within the transcode directory
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64,
    /// Was the file resized?
    pub resized: Option<bool>,
    /// Size of the additional file before resizing in bytes
    pub source_size: Option<u64>,
    /// Resize command
    pub resize_command: Option<String>,
}
