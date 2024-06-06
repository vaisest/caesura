use serde::Deserialize;

use crate::api::{Group, Torrent};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponse {
    pub group: Group,
    pub torrents: Vec<Torrent>,
}
