use colored::Colorize;
use di::{injectable, Ref};
use log::{debug, info};

use crate::errors::AppError;
use crate::fs::*;
use crate::jobs::JobRunner;
use crate::logging::Colors;
use crate::source::Source;
use crate::spectrogram::*;

/// Generate spectrograms for each track of a FLAC source.
#[injectable]
pub struct SpectrogramCommand {
    paths: Ref<PathManager>,
    factory: Ref<SpectrogramJobFactory>,
    runner: Ref<JobRunner>,
}

impl SpectrogramCommand {
    /// Generate spectrogram images from flac files.
    pub async fn execute(&self, source: &Source) -> Result<bool, AppError> {
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
