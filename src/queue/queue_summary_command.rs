use crate::errors::AppError;
use crate::options::{CacheOptions, Options};
use crate::queue::Queue;
use crate::queue::QueueSummary;
use crate::spectrogram::SpectrogramStatus;
use crate::transcode::TranscodeStatus;
use crate::upload::UploadStatus;
use crate::verify::VerifyStatus;
use di::{injectable, Ref, RefMut};

/// List the sources in the queue
#[injectable]
pub struct QueueSummaryCommand {
    cache_options: Ref<CacheOptions>,
    queue: RefMut<Queue>,
}

impl QueueSummaryCommand {
    pub fn execute_cli(&mut self) -> Result<bool, AppError> {
        if !self.cache_options.validate() {
            return Ok(false);
        }
        let summary = self.execute()?;
        let yaml = serde_yaml::to_string(&summary)
            .or_else(|e| AppError::yaml(e, "serialize queue summary"))?;
        println!("{yaml}");
        Ok(true)
    }
    pub fn execute(&mut self) -> Result<QueueSummary, AppError> {
        let mut queue = self.queue.write().expect("Queue should be writeable");
        queue.load()?;
        let items = queue.get_all();
        let mut summary = QueueSummary::default();
        for item in items {
            summary.total += 1;
            match summary.indexer.get_mut(&item.indexer) {
                Some(count) => *count += 1,
                None => {
                    summary.indexer.insert(item.indexer.clone(), 1);
                }
            }
            match item.verify {
                None => summary.verify_none += 1,
                Some(VerifyStatus { verified: true, .. }) => summary.verify_verified_true += 1,
                Some(VerifyStatus {
                    verified: false, ..
                }) => summary.verify_verified_false += 1,
            };
            match item.spectrogram {
                None => summary.spectrogram_none += 1,
                Some(SpectrogramStatus { success: true, .. }) => {
                    summary.spectrogram_success_true += 1;
                }
                Some(SpectrogramStatus { success: false, .. }) => {
                    summary.spectrogram_success_false += 1;
                }
            };
            match item.transcode {
                None => summary.transcode_none += 1,
                Some(TranscodeStatus { success: true, .. }) => summary.transcode_success_true += 1,
                Some(TranscodeStatus { success: false, .. }) => {
                    summary.transcode_success_false += 1
                }
            };
            match item.upload {
                None => summary.upload_none += 1,
                Some(UploadStatus { success: true, .. }) => summary.upload_success_true += 1,
                Some(UploadStatus { success: false, .. }) => summary.upload_success_false += 1,
            };
        }
        Ok(summary)
    }
}
