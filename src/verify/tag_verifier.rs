use crate::errors::AppError;
use crate::fs::{convert_to_id3v2, get_vorbis_tags, replace_vinyl_track_numbering, FlacFile};
use crate::source::Source;
use lofty::prelude::Accessor;
use lofty::prelude::ItemKey::Composer;

pub struct TagVerifier;

impl TagVerifier {
    pub fn execute(flac: &FlacFile, source: &Source) -> Result<Vec<String>, AppError> {
        let mut tags = get_vorbis_tags(flac)?;
        convert_to_id3v2(&mut tags);
        let _ = replace_vinyl_track_numbering(&mut tags);
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
        if is_classical && tags.get(&Composer).is_none() {
            missing.push("composer".to_owned());
        }
        if tags.track().is_none() {
            missing.push("track_number".to_owned());
        }
        Ok(missing)
    }
}
