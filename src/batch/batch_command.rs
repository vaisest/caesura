use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{debug, info, trace, warn};

use crate::batch::{BatchCache, BatchItem};
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

        let mut cache = match &self.batch_options.cache {
            None => BatchCache::new(),
            Some(path) => BatchCache::from_file(path)?,
        };
        let source = self.shared_options.get_value(|x| x.source.clone());
        cache.load_from(source)?;
        let queue = cache.get_queue();
        debug!("{} {} sources", "Processing".bold(), queue.len());
        let skip_spectrogram = self.batch_options.get_value(|x| x.no_spectrogram);
        let skip_upload = self.batch_options.get_value(|x| x.no_upload);
        let mut count = 0;
        for item in queue {
            let id = match self.id_provider.get_by_file(&item.path).await {
                Ok(id) => id,
                Err(error) => {
                    cache.update(&item.path, |item| item.set_skipped(error.to_string()));
                    trace!("{error}");
                    continue;
                }
            };
            let source = match self.get_source(id).await {
                Ok(source) => source,
                Err(error) => {
                    cache.update(&item.path, |item| item.set_failed(error.to_string()));
                    warn!("{error}");
                    continue;
                }
            };
            if let Some(reason) = self.verify(&source).await? {
                cache.update(&item.path, |item| item.set_skipped(reason.to_string()));
                continue;
            }
            if !skip_spectrogram {
                self.spectrogram.execute_internal(&source).await?;
            }
            if !self.transcode.execute_internal(&source).await? {
                cache.update(&item.path, |item| {
                    item.set_failed("transcode failed".to_owned());
                });
                continue;
            }
            if !skip_upload {
                if self
                    .upload
                    .write()
                    .expect("UploadCommand should be writeable")
                    .execute_internal(&source)
                    .await?
                {
                    cache.update(&item.path, BatchItem::set_uploaded);
                } else {
                    cache.update(&item.path, |item| {
                        item.set_failed("upload failed".to_owned());
                    });
                    continue;
                }
            }
            count += 1;
            if let Some(limit) = self.batch_options.limit {
                if count >= limit {
                    info!("{} batch limit: {limit}", "Reached".bold());
                    break;
                }
            }
        }
        if let Some(path) = &self.batch_options.cache {
            cache.write(path)?;
        }
        info!("{} batch process of {count} items", "Completed".bold());
        Ok(true)
    }

    async fn get_source(&mut self, id: i64) -> Result<Source, AppError> {
        self.source_provider
            .write()
            .expect("SourceProvider should be writable")
            .get(id)
            .await
    }

    async fn verify(&mut self, source: &Source) -> Result<Option<String>, AppError> {
        let errors: Vec<String> = self
            .verify
            .write()
            .expect("VerifyCommand should be writeable")
            .execute_internal(source)
            .await?
            .iter()
            .map(ToString::to_string)
            .collect();
        if errors.is_empty() {
            Ok(None)
        } else {
            let reason = errors.join(". ");
            Ok(Some(reason))
        }
    }
}
