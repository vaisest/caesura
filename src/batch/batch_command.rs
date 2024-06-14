use std::path::PathBuf;

use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{debug, info, warn};

use crate::errors::AppError;
use crate::options::{
    BatchOptions, FileOptions, Options, SharedOptions, SpectrogramOptions, TargetOptions,
    VerifyOptions,
};
use crate::source::*;
use crate::spectrogram::SpectrogramCommand;
use crate::transcode::TranscodeCommand;
use crate::upload::UploadCommand;
use crate::verify::VerifyCommand;

/// Batch a FLAC source is suitable for transcoding.
#[injectable]
pub struct BatchCommand {
    shared_options: Ref<SharedOptions>,
    verify_options: Ref<VerifyOptions>,
    target_options: Ref<TargetOptions>,
    spectrogram_options: Ref<SpectrogramOptions>,
    file_options: Ref<FileOptions>,
    batch_options: Ref<BatchOptions>,
    id_provider: Ref<IdProvider>,
    source_provider: RefMut<SourceProvider>,
    verify: RefMut<VerifyCommand>,
    spectrogram: Ref<SpectrogramCommand>,
    transcode: Ref<TranscodeCommand>,
    upload: RefMut<UploadCommand>,
}

impl BatchCommand {
    pub async fn execute(&mut self) -> Result<bool, AppError> {
        if !self.shared_options.validate()
            || !self.verify_options.validate()
            || !self.target_options.validate()
            || !self.spectrogram_options.validate()
            || !self.file_options.validate()
            || !self.batch_options.validate()
        {
            return Ok(false);
        }
        let source_directory = self.shared_options.get_value(|x| x.source.clone());
        let source_directory = PathBuf::from(source_directory);
        let ids: Vec<i64> = self.id_provider.get_by_directory(&source_directory).await?;
        debug!("{} {} sources", "Processing".bold(), ids.len());
        let skip_spectrogram = self.batch_options.get_value(|x| x.no_spectrogram);
        let skip_upload = self.batch_options.get_value(|x| x.no_upload);
        let mut count = 0;
        for id in ids {
            let Some(source) = self.get_source(id).await else {
                continue;
            };
            if !self.verify(&source).await? {
                continue;
            }
            if !skip_spectrogram {
                self.spectrogram.execute_internal(&source).await?;
            }
            if !self.transcode.execute_internal(&source).await? {
                continue;
            }
            if !skip_upload {
                self.upload
                    .write()
                    .expect("UploadCommand should be writeable")
                    .execute_internal(&source)
                    .await?;
            }
            count += 1;
            if let Some(limit) = self.batch_options.limit {
                if count >= limit {
                    info!("{} batch limit: {limit}", "Reached".bold());
                    break;
                }
            }
        }
        info!("{} batch process of {count} items", "Completed".bold());
        Ok(true)
    }

    async fn get_source(&mut self, id: i64) -> Option<Source> {
        let result = self
            .source_provider
            .write()
            .expect("SourceProvider should be writable")
            .get(id)
            .await;
        match result {
            Ok(source) => Some(source),
            Err(error) => {
                warn!("{} {error}", "Skipping".bold());
                None
            }
        }
    }

    async fn verify(&mut self, source: &Source) -> Result<bool, AppError> {
        let errors = self
            .verify
            .write()
            .expect("VerifyCommand should be writeable")
            .execute_internal(source)
            .await?;
        if errors.is_empty() {
            Ok(true)
        } else {
            let error = errors.first().expect("should be at least one error");
            debug!("{} {error} {source}", "Skipped".bold());
            Ok(false)
        }
    }
}
