use crate::fs::FlacFile;
use crate::naming::Sanitizer;

pub struct TrackName;

impl TrackName {
    pub fn get(flac: &FlacFile) -> Option<String> {
        let tags = flac.get_tags().ok()?;
        let track_number = tags.track_number()?;
        let title = tags.title()?;
        let file_name = format!("{track_number:0>2} {title}");
        Some(Sanitizer::execute(file_name))
    }
}
