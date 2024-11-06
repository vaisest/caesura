use html_escape::decode_html_entities;

use gazelle_api::{Group, Torrent};

#[derive(Clone, Debug)]
pub struct Metadata {
    pub artist: String,
    pub album: String,
    pub remaster_title: String,
    pub year: u16,
    pub media: String,
}

impl Metadata {
    #[must_use]
    pub fn new(group: &Group, torrent: &Torrent) -> Self {
        Metadata {
            artist: get_artist(group),
            album: get_album(group),
            remaster_title: get_remaster_title(torrent),
            year: get_year(group, torrent),
            media: torrent.media.clone(),
        }
    }
}

fn get_artist(group: &Group) -> String {
    let artist = get_artist_internal(group).unwrap_or("Unknown Artist".to_owned());
    decode_html_entities(&artist).to_string()
}

fn get_artist_internal(group: &Group) -> Option<String> {
    let info = group.music_info.clone()?;
    if info.artists.len() > 1 {
        Some("Various Artists".to_owned())
    } else if info.artists.is_empty() {
        None
    } else {
        info.artists.into_iter().map(|x| x.name).next()
    }
}

fn get_album(group: &Group) -> String {
    decode_html_entities(&group.name).to_string()
}

fn get_remaster_title(torrent: &Torrent) -> String {
    decode_html_entities(&torrent.remaster_title).to_string()
}

fn get_year(group: &Group, torrent: &Torrent) -> u16 {
    if torrent.remaster_year.is_none() || torrent.remaster_year == Some(0) {
        group.year
    } else {
        torrent.remaster_year.expect("Remaster year should be set")
    }
}

#[cfg(test)]
mod tests {
    use crate::source::metadata::get_artist;
    use gazelle_api::{Artist, Group, MusicInfo};

    #[test]
    fn get_artist_none() {
        // Arrange
        let expected = "Unknown Artist".to_owned();
        let group = Group {
            music_info: Some(MusicInfo {
                artists: Vec::new(),
                ..MusicInfo::default()
            }),
            ..Group::default()
        };

        // Act
        let artist = get_artist(&group);

        // Assert
        assert_eq!(artist, expected);
    }

    #[test]
    fn get_artist_one() {
        // Arrange
        let expected = "Hello, world!".to_owned();
        let group = Group {
            music_info: Some(MusicInfo {
                artists: vec![Artist {
                    id: 12345,
                    name: expected.clone(),
                }],
                ..MusicInfo::default()
            }),
            ..Group::default()
        };

        // Act
        let artist = get_artist(&group);

        // Assert
        assert_eq!(artist, expected);
    }

    #[test]
    fn get_artist_two() {
        // Arrange
        let expected = "Various Artists".to_owned();
        let group = Group {
            music_info: Some(MusicInfo {
                artists: vec![
                    Artist {
                        id: 12345,
                        name: "Artist One".to_owned(),
                    },
                    Artist {
                        id: 12345,
                        name: "Artist Two".to_owned(),
                    },
                ],
                ..MusicInfo::default()
            }),
            ..Group::default()
        };

        // Act
        let artist = get_artist(&group);

        // Assert
        assert_eq!(artist, expected);
    }
}
