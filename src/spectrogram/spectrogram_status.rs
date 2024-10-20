use crate::errors::AppError;
use crate::queue::TimeStamp;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct SpectrogramStatus {
    /// Did the spectrogram command succeed?
    pub success: bool,
    /// Path to the spectrogram directory
    pub path: Option<PathBuf>,
    /// Number of spectrograms created
    pub count: usize,
    /// Time the spectrogram completed
    pub completed: TimeStamp,
    /// Error message if the spectrogram failed
    pub error: Option<AppError>,
}
