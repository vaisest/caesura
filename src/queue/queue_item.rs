use crate::imdl::TorrentSummary;
use crate::source::get_torrent_id_from_torrent_url_relaxed;
use crate::spectrogram::SpectrogramStatus;
use crate::transcode::TranscodeStatus;
use crate::upload::UploadStatus;
use crate::verify::VerifyStatus;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct QueueItem {
    /// Source name
    pub name: String,
    /// Torrent file path
    pub path: PathBuf,
    /// Source info hash
    pub hash: String,
    /// Source indexer
    pub indexer: String,
    /// Source id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Reason for skipping?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<String>,
    /// Verification status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<VerifyStatus>,
    /// Transcode status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectrogram: Option<SpectrogramStatus>,
    /// Transcode status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcode: Option<TranscodeStatus>,
    /// Upload status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<UploadStatus>,
}

impl QueueItem {
    /// Create a new [`QueueItem`] from a [`TorrentSummary`]
    ///
    /// Returns `None` if the torrent does not have a source or comment
    /// or the comment does not contain a torrent id
    #[must_use]
    pub fn from_torrent(path: PathBuf, torrent: TorrentSummary) -> Self {
        let comment = torrent.comment.unwrap_or_default();
        // TODO It's possible (but highly unlikely) that the URL does not match the indexer
        let id = get_torrent_id_from_torrent_url_relaxed(&comment);
        Self {
            name: torrent.name,
            path,
            hash: torrent.info_hash,
            indexer: torrent.source.unwrap_or_default().to_lowercase(),
            id,
            ..Self::default()
        }
    }
}

impl Display for QueueItem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.name)
    }
}
