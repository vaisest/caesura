use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadResponse {
    pub private: bool,
    pub source: bool,
    #[serde(rename = "requestid")]
    pub request_id: Option<i64>,
    #[serde(rename = "torrentid")]
    pub torrent_id: i64,
    #[serde(rename = "groupid")]
    pub group_id: i64,
}
