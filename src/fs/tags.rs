use crate::errors::error;
use crate::fs::FlacFile;
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::ItemKey::TrackNumber;
use lofty::tag::{Accessor, Tag, TagType};
use log::trace;
use regex::Regex;
use rogue_logging::Error;

pub(crate) fn get_vorbis_tags(flac: &FlacFile) -> Result<Tag, Error> {
    let file = Probe::open(flac.path.clone())
        .map_err(|e| error("get tags", e.to_string()))?
        .read()
        .map_err(|e| error("get tags", e.to_string()))?;
    if let Some(vorbis) = file.tag(TagType::VorbisComments) {
        Ok(vorbis.clone())
    } else {
        Err(error(
            "get tags",
            format!("No Vobis comments: {}", flac.path.display()),
        ))
    }
}

pub(crate) fn convert_to_id3v2(tags: &mut Tag) {
    tags.re_map(TagType::Id3v2);
}

pub(crate) fn replace_vinyl_track_numbering(tags: &mut Tag) -> Result<(), Error> {
    let track = tags.get_string(&TrackNumber).ok_or_else(|| {
        error(
            "replace vinyl track numbering",
            "No track number string".to_owned(),
        )
    })?;
    let (disc_number, track_number) = get_numeric_from_vinyl_format(track).ok_or_else(|| {
        error(
            "replace vinyl track numbering",
            "Not vinyl format".to_owned(),
        )
    })?;
    trace!(
        "Replacing vinyl track ({track}) with numeric: track {track_number}, disc {disc_number}"
    );
    tags.set_disk(disc_number);
    tags.set_track(track_number);
    Ok(())
}

pub(crate) fn get_numeric_from_vinyl_format(input: &str) -> Option<(u32, u32)> {
    let re = Regex::new(r"^([A-Z])(\d+)$").ok()?;
    let captures = re.captures(input)?;
    let disc_letter = captures.get(1)?.as_str().chars().next()?;
    let track_number: u32 = captures.get(2)?.as_str().parse().ok()?;
    let disc_number = letter_to_number(disc_letter)?;
    Some((disc_number, track_number))
}

#[allow(clippy::as_conversions)]
fn letter_to_number(letter: char) -> Option<u32> {
    match letter {
        'A'..='Z' => Some((letter as u32) - ('A' as u32) + 1),
        _ => None,
    }
}
