use std::collections::BTreeSet;
use std::fmt;
use std::path::PathBuf;

use colored::Colorize;

use crate::api::{Group, Torrent};
use crate::formats::existing_format::ExistingFormat;
use crate::formats::SourceFormat;
use crate::naming::SourceName;
use crate::source::metadata::Metadata;
use rogue_logging::Colors;

/// Source to be transcoded
#[derive(Debug)]
pub struct Source {
    pub torrent: Torrent,

    pub group: Group,

    pub existing: BTreeSet<ExistingFormat>,

    pub format: SourceFormat,

    pub directory: PathBuf,

    pub metadata: Metadata,
}

impl fmt::Display for Source {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        SourceName::get(&self.metadata)
            .gray()
            .italic()
            .fmt(formatter)
    }
}
