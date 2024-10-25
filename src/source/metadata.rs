use html_escape::decode_html_entities;

use crate::api::{Group, Torrent};

#[derive(Clone, Debug)]
pub struct Metadata {
    pub artist: String,
    pub album: String,
    pub remaster_title: String,
    pub year: u16,
    pub media: String,
}

impl Metadata {
    #[allow(
        clippy::cast_sign_loss,
        clippy::as_conversions,
        clippy::cast_possible_truncation
    )]
    #[must_use]
    pub fn new(group: &Group, torrent: &Torrent) -> Self {
        let artist = decode_html_entities(Self::get_artist(group)).to_string();
        let album = decode_html_entities(&group.name).to_string();
        let remaster_title = decode_html_entities(&torrent.remaster_title).to_string();
        let year = if torrent.remaster_year.is_none() || torrent.remaster_year == Some(0) {
            group.year as u16
        } else {
            torrent.remaster_year.expect("Remaster year should be set") as u16
        };
        let media = torrent.media.clone();
        Metadata {
            artist,
            album,
            remaster_title,
            year,
            media,
        }
    }

    fn get_artist(group: &Group) -> &str {
        match &group.music_info {
            None => "Unknown Artist",
            Some(info) => {
                if info.artists.len() > 1 {
                    "Various Artists"
                } else if info.artists.is_empty() {
                    "Unknown Artist"
                } else {
                    info.artists
                        .first()
                        .expect("should be at least one artist")
                        .name
                        .as_str()
                }
            }
        }
    }
}
