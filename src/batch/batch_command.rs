use std::path::PathBuf;

use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{debug, warn};

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
        let sources = self
            .source_provider
            .write()
            .expect("SourceProvider should be writeable")
            .get_by_directory(&source_directory)
            .await?;
        let verified = self.verify(sources).await?;
        self.create_spectrograms(&verified).await?;
        let transcoded = self.transcode(verified).await?;
        if self.batch_options.get_value(|x| x.no_upload) {
            debug!("{} upload due to settings", "Skipped".bold());
            return Ok(true);
        }
        self.upload(transcoded).await?;
        Ok(true)
    }

    async fn verify(&mut self, sources: Vec<Source>) -> Result<Vec<Source>, AppError> {
        let mut verified: Vec<Source> = Vec::new();
        for source in sources {
            let errors = self
                .verify
                .write()
                .expect("VerifyCommand should be writeable")
                .execute_internal(&source)
                .await?;
            if errors.is_empty() {
                verified.push(source);
            } else {
                let error = errors.first().expect("should be at least one error");
                debug!("{} {error} {source}", "Skipped".bold());
            }
        }
        Ok(verified)
    }

    async fn create_spectrograms(&mut self, verified: &Vec<Source>) -> Result<(), AppError> {
        if !self.batch_options.get_value(|x| x.no_spectrogram) {
            for source in verified {
                self.spectrogram.execute_internal(source).await?;
            }
        }
        Ok(())
    }

    async fn transcode(&mut self, verified: Vec<Source>) -> Result<Vec<Source>, AppError> {
        let mut transcoded: Vec<Source> = Vec::new();
        for source in verified {
            let is_transcoded = self.transcode.execute_internal(&source).await?;
            if is_transcoded {
                transcoded.push(source);
            } else {
                warn!("{} to transcode {source}", "Failed".bold());
            }
        }
        Ok(transcoded)
    }

    async fn upload(&mut self, transcoded: Vec<Source>) -> Result<(), AppError> {
        let mut uploaded: Vec<Source> = Vec::new();
        for source in transcoded {
            let is_uploaded = self
                .upload
                .write()
                .expect("UploadCommand should be writeable")
                .execute_internal(&source)
                .await?;
            if is_uploaded {
                uploaded.push(source);
            } else {
                warn!("{} to upload {source}", "Failed".bold());
            }
        }
        Ok(())
    }
}
