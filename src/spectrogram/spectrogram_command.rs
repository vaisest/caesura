use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{debug, error, info};

use crate::errors::AppError;
use crate::fs::*;
use crate::jobs::JobRunner;
use crate::options::{Options, SharedOptions, SpectrogramOptions};
use crate::queue::TimeStamp;
use crate::source::{Source, SourceProvider};
use crate::spectrogram::*;

/// Generate spectrograms for each track of a FLAC source.
#[injectable]
pub struct SpectrogramCommand {
    shared_options: Ref<SharedOptions>,
    spectrogram_options: Ref<SpectrogramOptions>,
    source_provider: RefMut<SourceProvider>,
    paths: Ref<PathManager>,
    factory: Ref<SpectrogramJobFactory>,
    runner: Ref<JobRunner>,
}

impl SpectrogramCommand {
    /// Generate spectrogram images from flac files.
    pub async fn execute_cli(&self) -> Result<bool, AppError> {
        if !self.shared_options.validate() || !self.spectrogram_options.validate() {
            return Ok(false);
        }
        let source = self
            .source_provider
            .write()
            .expect("Source provider should be writeable")
            .get_from_options()
            .await?;
        let status = self.execute(&source).await;
        if let Some(error) = &status.error {
            error!("{error}");
        }
        Ok(status.success)
    }

    /// Generate spectrogram images from flac files.
    #[must_use]
    pub async fn execute(&self, source: &Source) -> SpectrogramStatus {
        info!("{} spectrograms for {}", "Creating".bold(), source);
        let collection = Collector::get_flacs(&source.directory);
        let jobs = self.factory.create(&collection, source);
        let count = jobs.len();
        self.runner.add(jobs);
        match self.runner.execute().await {
            Ok(()) => {
                info!("{} {count} spectrograms for {source}", "Created".bold());
                let path = self.paths.get_spectrogram_dir(source);
                let path_display = path.to_string_lossy().to_string();
                debug!("in {path_display}");
                SpectrogramStatus {
                    success: true,
                    path: Some(path),
                    count,
                    completed: TimeStamp::now(),
                    error: None,
                }
            }
            Err(error) => SpectrogramStatus {
                success: false,
                path: None,
                count,
                completed: TimeStamp::now(),
                error: Some(error),
            },
        }
    }
}
