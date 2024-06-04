use colored::Colorize;
use log::info;
use crate::source::{Metadata, Source};
use regex::Regex;
use crate::fs::FlacFile;
use crate::logging::Colors;
use crate::naming::{SourceName, TrackName};

pub struct Shortener;

impl Shortener {
    #[must_use]
    pub fn shorten_album(metadata: &Metadata) -> Option<Metadata> {
        let result = remove_parenthetical_suffix(&metadata.album);
        match result {
            None => None,
            Some(album) => {
                let mut metadata = metadata.clone();
                metadata.album = album;
                Some(metadata)
            }
        }
    }
    
    pub fn suggest_track_name(flac: &FlacFile) {
        if let Some(file_name) = TrackName::get(&flac) {
            let difference = flac.file_name.len() - file_name.len();
            if difference > 0 {
                info!("{} track could save {difference} characters: {}", "Renaming".bold(), file_name.gray());
            }
        }
    }
    
    pub fn suggest_album_name(source: &Source) {
        if let Some(shortened) = Shortener::shorten_album(&source.metadata) {
            let before = SourceName::from_metadata(&source.metadata);
            let after = SourceName::from_metadata(&shortened);
            let difference = before.len() - after.len();
            if difference > 0 {
                info!("{} directory could save {difference} characters: {}", "Renaming".bold(), after.gray());
            }
        }
    }
}

fn remove_parenthetical_suffix(input: &str) -> Option<String> {
    let result = Regex::new(r"^(.*)(\(.*\))$")
        .expect("Regex should compile")
        .captures(input);
    match result {
        None => None,
        Some(captures) => {
            let shortened = captures.get(1).expect("Should have captures").as_str();
            let shortened = shortened.trim();
            if shortened.len() > 4 {
                Some(shortened.to_owned())
            } else {
                None
            }
        }
    }
}
