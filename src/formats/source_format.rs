use crate::formats::ExistingFormat;

/// Format of a [Source].
#[derive(Clone, Copy, Debug)]
pub enum SourceFormat {
    Flac24,
    Flac,
}

impl SourceFormat {
    #[must_use]
    pub fn to_existing(self) -> ExistingFormat {
        match self {
            SourceFormat::Flac24 => ExistingFormat::Flac24,
            SourceFormat::Flac => ExistingFormat::Flac,
        }
    }
    #[must_use]
    pub fn get_title(&self) -> &str {
        match self {
            SourceFormat::Flac24 => "FLAC 24bit Lossless",
            SourceFormat::Flac => "FLAC Lossless",
        }
    }
}
