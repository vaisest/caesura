use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
