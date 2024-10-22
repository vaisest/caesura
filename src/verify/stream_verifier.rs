use crate::fs::FlacFile;
use crate::source::SourceIssue;
use crate::source::SourceIssue::*;
use crate::transcode::get_resample_rate;

pub struct StreamVerifier;

impl StreamVerifier {
    pub fn execute(flac: &FlacFile) -> Vec<SourceIssue> {
        let mut errors = Vec::new();
        let info = match flac.get_stream_info() {
            Ok(info) => info,
            Err(claxon_error) => {
                errors.push(FlacError {
                    path: flac.path.clone(),
                    error: format!("{claxon_error}"),
                });
                return errors;
            }
        };
        if get_resample_rate(&info).is_none() {
            errors.push(SampleRate {
                path: flac.path.clone(),
                rate: info.sample_rate,
            });
        }
        if info.channels > 2 {
            errors.push(Channels {
                path: flac.path.clone(),
                count: info.channels,
            });
        }
        errors
    }
}
