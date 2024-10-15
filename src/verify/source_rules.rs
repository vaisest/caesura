use crate::verify::SourceRule::*;
use std::fmt::{Display, Formatter};

pub const MAX_PATH_LENGTH: usize = 180;

pub enum SourceRule {
    IncorrectCategory(String),
    SceneNotSupported,
    LossyMasterNeedsApproval,
    LossyWebNeedsApproval,
    TrumpableNotSuitable,
    NoTranscodeFormats,
    SourceDirectoryNotFound(String),
    NoFlacFiles(String),
    IncorrectHash(String),
    PathTooLong(String),
    NoArtistTag(String),
    NoAlbumTag(String),
    NoTitleTag(String),
    NoComposerTag(String),
    NoTrackNumberTag(String),
    FlacIOError(String, String),
    FlacFormatError(String, String),
    FlacUnsupported(String, String),
    UnknownSampleRate(u32, String),
    TooManyChannels(u32, String),
}

impl Display for SourceRule {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            IncorrectCategory(category) => format!("Category was not Music: {category}"),
            SceneNotSupported => "Scene releases are not supported".to_owned(),
            LossyMasterNeedsApproval => "Lossy master releases need approval".to_owned(),
            LossyWebNeedsApproval => "Lossy web releases need approval".to_owned(),
            TrumpableNotSuitable => "Source is trumpable".to_owned(),
            NoTranscodeFormats => "All allowed formats have been transcoded to already".to_owned(),
            SourceDirectoryNotFound(path) => format!("Source directory not found: {path}"),
            NoFlacFiles(path) => format!("No Flac files found in source directory: {path}"),
            IncorrectHash(details) => format!("Files do not match hash:\n{details}"),
            PathTooLong(path) => format!(
                "Path is {} characters longer than allowed: {path}",
                path.len() - MAX_PATH_LENGTH
            ),
            NoArtistTag(path) => format!("No artist tag: {path}"),
            NoAlbumTag(path) => format!("No album tag: {path}"),
            NoTitleTag(path) => format!("No title tag: {path}"),
            NoComposerTag(path) => format!("No composer tag: {path}"),
            NoTrackNumberTag(path) => format!("No track number tag: {path}"),
            UnknownSampleRate(rate, path) => format!("Unknown sample rate: {rate}: {path}"),
            TooManyChannels(channels, path) => {
                format!("Unable to transcode more than two channels: {channels}: {path}")
            }
            FlacIOError(message, path) => format!("IO error: {message}: {path}"),
            FlacFormatError(message, path) => format!("Format error: {message}: {path}"),
            FlacUnsupported(message, path) => format!("Unsupported error: {message}: {path}"),
        };
        message.fmt(formatter)
    }
}
