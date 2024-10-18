use crate::errors::AppError;
use crate::fs::FlacFile;
use crate::source::Source;
use log::warn;

pub struct TagVerifier;

impl TagVerifier {
    pub fn execute(flac: &FlacFile, source: &Source) -> Result<Vec<String>, AppError> {
        let tags = flac.get_tags()?;
        let mut missing: Vec<String> = Vec::new();
        if tags.artist().is_none() {
            missing.push("artist".to_owned());
        }
        if tags.album().is_none() {
            missing.push("album".to_owned());
        }
        if tags.title().is_none() {
            missing.push("title".to_owned());
        }
        let is_classical = source.group.tags.contains(&"classical".to_owned());
        if is_classical && tags.composer().is_none() {
            missing.push("composer".to_owned());
        }
        if tags.track_number().is_none() {
            let is_vinyl = source.metadata.media.eq_ignore_ascii_case("vinyl");
            if is_vinyl {
                warn!("Unable to verify if the track number is valid. Vinyl releases can have non-standard track numbers (e.g. A1, A2, etc).");
            } else {
                missing.push("track_number".to_owned());
            }
        }
        Ok(missing)
    }
}
