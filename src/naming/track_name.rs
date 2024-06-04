use crate::fs::FlacFile;
use crate::naming::FORBIDDEN_CHARACTERS;

pub struct TrackName;

impl TrackName {
    pub fn get(flac: &FlacFile) -> Option<String> {
        let tags = flac.get_tags().ok()?;
        let track_number = tags.track_number()?;
        let title = tags.title()?.replace(&FORBIDDEN_CHARACTERS[..], "_");
        Some(format!("{track_number:0>2} {title}"))
    }
}