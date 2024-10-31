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
