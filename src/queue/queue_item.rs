use crate::db::Hash;
use crate::imdl::TorrentSummary;
use crate::source::get_torrent_id_from_torrent_url;
use crate::spectrogram::SpectrogramStatus;
use crate::transcode::TranscodeStatus;
use crate::upload::UploadStatus;
use crate::verify::VerifyStatus;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct QueueItem {
    /// Source name
    pub name: String,
    /// Torrent file path
    pub path: PathBuf,
    /// Source info hash
    pub hash: Hash<20>,
    /// Source indexer
    pub indexer: String,
    /// Source id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
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
        let id = get_torrent_id_from_torrent_url(&comment);
        Self {
            name: torrent.name,
            path,
            hash: Hash::from_string(&torrent.info_hash).expect("torrent hash should be valid"),
            indexer: torrent.source.unwrap_or_default().to_lowercase(),
            id,
            ..Self::default()
        }
    }
}

impl Display for QueueItem {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.name)
    }
}
