use audiotags::Id3v2Tag;
use di::{injectable, Ref};

use crate::errors::AppError;
use crate::formats::target_format::TargetFormat;
use crate::fs::{FlacFile, PathManager};
use crate::jobs::Job;
use crate::source::Source;
use crate::transcode::transcode_job::TranscodeJob;
use crate::transcode::*;

#[injectable]
pub struct TranscodeJobFactory {
    paths: Ref<PathManager>,
}

impl TranscodeJobFactory {
    /// Create a [`TranscodeJob`] for each [`FlacFile`] in the [`Vec<FlacFile>`].
    pub fn create(
        &self,
        flacs: &[FlacFile],
        source: &Source,
        format: TargetFormat,
    ) -> Result<Vec<Job>, AppError> {
        let mut jobs = Vec::new();
        for (index, flac) in flacs.iter().enumerate() {
            jobs.push(self.create_single(index, flac, source, format)?);
        }
        Ok(jobs)
    }

    /// Create a single [`TranscodeJob`] from a `flac_file`.
    pub fn create_single(
        &self,
        index: usize,
        flac: &FlacFile,
        source: &Source,
        format: TargetFormat,
    ) -> Result<Job, AppError> {
        let info = flac.get_stream_info()?;
        let id = format!("Transcode {format:<7?}{index:>3}");
        let output_path = self.paths.get_transcode_path(source, &format, flac);
        let commands = if matches!(format, TargetFormat::Flac) && is_resample_required(&info) {
            let cmd = CommandFactory::new_flac_resample(flac, &info, &output_path)?;
            vec![cmd]
        } else {
            let decode_cmd = CommandFactory::new_decode(flac, &info)?;
            let encode_cmd = CommandFactory::new_encode(format, &output_path);
            vec![decode_cmd, encode_cmd]
        };
        let tags = if matches!(format, TargetFormat::_320) || matches!(format, TargetFormat::V0) {
            let tags = flac.get_tags()?;
            Some(Id3v2Tag::from(tags))
        } else {
            None
        };
        Ok(Job::Transcode(TranscodeJob {
            id,
            output_path,
            commands,
            tags,
        }))
    }
}
