use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{debug, info};

use crate::errors::AppError;
use crate::fs::*;
use crate::jobs::JobRunner;
use crate::logging::Colors;
use crate::options::{Options, SharedOptions, SpectrogramOptions};
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
    pub async fn execute(&self) -> Result<bool, AppError> {
        if !self.shared_options.validate() || !self.spectrogram_options.validate() {
            return Ok(false);
        }
        let source = self
            .source_provider
            .write()
            .expect("Source provider should be writeable")
            .get()
            .await?;
        self.execute_internal(&source).await
    }

    /// Generate spectrogram images from flac files.
    async fn execute_internal(&self, source: &Source) -> Result<bool, AppError> {
        info!("{} spectrograms for {}", "Creating".bold(), source);
        let collection = Collector::get_flacs(&source.directory);
        let jobs = self.factory.create(&collection, source);
        let count = jobs.len();
        self.runner.add(jobs);
        self.runner.execute().await?;
        info!("{} {} spectrograms for {}", "Created".bold(), count, source);
        let output_dir = self.paths.get_spectrogram_dir(source);
        debug!(
            "{} {}",
            "in".gray(),
            output_dir.to_string_lossy().to_string().gray()
        );
        Ok(true)
    }
}
