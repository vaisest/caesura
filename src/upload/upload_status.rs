use crate::errors::AppError;
use crate::formats::TargetFormat;
use crate::queue::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct UploadStatus {
    /// Did the upload command succeed?
    pub success: bool,
    /// Uploaded formats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<UploadFormatStatus>>,
    /// Time the transcode completed
    pub completed: TimeStamp,
    /// Error messages
    ///
    /// It is possible for [`UploadCommand`] to succeed while still having errors.
    /// For example `copy_transcode_to_content_dir` and `copy_torrent_to` are recoverable,
    /// so may error but the upload still proceeds successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AppError>>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UploadFormatStatus {
    /// Transcode format
    pub format: TargetFormat,
    /// URL of the upload
    pub url: String,
}
