use crate::errors::AppError;
use crate::fs::FlacFile;
use crate::transcode::get_resample_rate;
use crate::verify::SourceRule;
use crate::verify::SourceRule::*;

pub struct StreamVerifier;

impl StreamVerifier {
    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(flac: &FlacFile) -> Result<Vec<SourceRule>, AppError> {
        let mut errors = Vec::new();
        let info = match flac.get_stream_info() {
            Ok(info) => info,
            Err(claxon_error) => {
                errors.push(FlacError {
                    path: flac.path.clone(),
                    error: format!("{claxon_error}"),
                });
                return Ok(errors);
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
        Ok(errors)
    }
}
