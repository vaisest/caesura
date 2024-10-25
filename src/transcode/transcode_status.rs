use crate::errors::AppError;
use crate::formats::TargetFormat;
use crate::queue::TimeStamp;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Deserialize, Serialize)]
pub struct TranscodeStatus {
    /// Did the transcode command succeed?
    pub success: bool,
    /// Transcode formats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<TranscodeFormatStatus>>,
    /// Additional files
    #[allow(dead_code)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional: Option<Vec<AdditionalStatus>>,
    /// Time the transcode completed
    pub completed: TimeStamp,
    /// Error message if the transcode failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AppError>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TranscodeFormatStatus {
    /// Did the transcode command succeed?
    pub format: TargetFormat,
    /// Path to the transcode directory
    pub path: PathBuf,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AdditionalStatus {
    /// Relative path of the additional file within the transcode directory
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64,
    /// Was the file resized?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resized: Option<bool>,
    /// Size of the additional file before resizing in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_size: Option<u64>,
    /// Resize command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_command: Option<String>,
}
