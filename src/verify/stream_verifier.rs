use crate::fs::FlacFile;
use crate::source::SourceIssue::*;
use crate::source::{SourceIssue, MAX_DURATION, MIN_BIT_RATE_KBPS};
use crate::transcode::{get_average_bit_rate, get_duration, get_resample_rate};

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
        if get_resample_rate(&info).is_err() {
            errors.push(SampleRate {
                path: flac.path.clone(),
                rate: info.sample_rate,
            });
        }
        if let Some(rate) = get_average_bit_rate(&info) {
            if rate < MIN_BIT_RATE_KBPS * 1000 {
                errors.push(BitRate {
                    path: flac.path.clone(),
                    rate,
                });
            }
        }
        if let Some(seconds) = get_duration(&info) {
            if seconds > MAX_DURATION {
                errors.push(Duration {
                    path: flac.path.clone(),
                    seconds,
                });
            }
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
