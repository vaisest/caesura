use crate::errors::AppError;
use crate::formats::TargetFormat;
use crate::queue::TimeStamp;

#[allow(dead_code)]
pub struct UploadStatus {
    /// Did the upload command succeed?
    pub success: bool,
    /// Uploaded formats
    pub formats: Option<Vec<UploadFormatStatus>>,
    /// Time the transcode completed
    pub completed: TimeStamp,
    /// Error messages
    ///
    /// It is possible for [`UploadCommand`] to succeed while still having errors.
    /// For example `copy_transcode_to_content_dir` and `copy_torrent_to` are recoverable,
    /// so may error but the upload still proceeds successfully.
    pub errors: Option<Vec<AppError>>,
}

#[allow(dead_code)]
pub struct UploadFormatStatus {
    /// Transcode format
    pub format: TargetFormat,
    /// URL of the upload
    pub url: String,
}
