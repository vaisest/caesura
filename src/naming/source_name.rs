use crate::naming::Sanitizer;
use crate::source::Metadata;

pub struct SourceName;

impl SourceName {
    #[must_use]
    pub fn get(metadata: &Metadata) -> String {
        let name = if metadata.remaster_title.is_empty() {
            format!(
                "{} - {} [{}]",
                metadata.artist, metadata.album, metadata.year
            )
        } else {
            format!(
                "{} - {} ({}) [{}]",
                metadata.artist, metadata.album, metadata.remaster_title, metadata.year
            )
        };
        Sanitizer::execute(name)
    }
}
