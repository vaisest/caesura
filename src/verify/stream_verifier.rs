use crate::errors::AppError;
use crate::fs::FlacFile;
use crate::transcode::get_resample_rate;
use crate::verify::SourceRule;
use crate::verify::SourceRule::*;
use claxon::Error;

pub struct StreamVerifier;

impl StreamVerifier {
    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(flac: &FlacFile) -> Result<Vec<SourceRule>, AppError> {
        let mut errors = Vec::new();
        let path = flac.get_path_string();
        let info = match flac.get_stream_info() {
            Ok(info) => info,
            Err(claxon_error) => {
                let error = match claxon_error {
                    Error::IoError(e) => FlacIOError(e.to_string(), path),
                    Error::FormatError(message) => FlacFormatError(message.to_owned(), path),
                    Error::Unsupported(message) => FlacUnsupported(message.to_owned(), path),
                };
                errors.push(error);
                return Ok(errors);
            }
        };
        if get_resample_rate(&info).is_none() {
            errors.push(UnknownSampleRate(info.sample_rate, path.clone()));
        }
        if info.channels > 2 {
            errors.push(TooManyChannels(info.channels, path));
        }
        Ok(errors)
    }
}
