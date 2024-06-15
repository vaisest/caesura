use std::fmt::Display;
use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::*;

use crate::errors::AppError;
use crate::formats::{TargetFormat, TargetFormatProvider};
use crate::fs::{Collector, PathManager};
use crate::imdl::ImdlCommand;
use crate::jobs::JobRunner;
use crate::logging::Colors;
use crate::options::{Options, SharedOptions, TargetOptions};
use crate::source::*;
use crate::transcode::{AdditionalJobFactory, TranscodeJobFactory};

/// Transcode each track of a FLAC source to the target formats.
#[injectable]
pub struct TranscodeCommand {
    shared_options: Ref<SharedOptions>,
    target_options: Ref<TargetOptions>,
    source_provider: RefMut<SourceProvider>,
    paths: Ref<PathManager>,
    targets: Ref<TargetFormatProvider>,
    transcode_job_factory: Ref<TranscodeJobFactory>,
    additional_job_factory: Ref<AdditionalJobFactory>,
    runner: Ref<JobRunner>,
}

impl TranscodeCommand {
    pub async fn execute(&self) -> Result<bool, AppError> {
        if !self.shared_options.validate() || !self.target_options.validate() {
            return Ok(false);
        }
        let source = self
            .source_provider
            .write()
            .expect("Source provider should be writeable")
            .get_from_options()
            .await?;
        self.execute_internal(&source).await
    }

    pub async fn execute_internal(&self, source: &Source) -> Result<bool, AppError> {
        let targets = self.targets.get(source.format, &source.existing);
        let targets = self.skip_completed(source, &targets);
        if targets.is_empty() {
            return Ok(true);
        }
        self.execute_transcode(source, &targets).await?;
        self.execute_additional(source, &targets).await?;
        self.execute_torrent(source, &targets).await?;
        Ok(true)
    }

    #[must_use]
    pub fn skip_completed(
        &self,
        source: &Source,
        targets: &Vec<TargetFormat>,
    ) -> Vec<TargetFormat> {
        let mut out: Vec<TargetFormat> = Vec::new();
        for target in targets {
            let path = self.paths.get_torrent_path(source, target);
            if path.exists() {
                debug!(
                    "{} {target} as it has already been transcoded.",
                    "Skipping".bold()
                );
                trace!("{path:?}");
            } else {
                out.push(*target);
            }
        }
        out
    }

    pub async fn execute_transcode(
        &self,
        source: &Source,
        targets: &Vec<TargetFormat>,
    ) -> Result<(), AppError> {
        let flacs = Collector::get_flacs(&source.directory);
        info!(
            "{} to {} for {} FLACs in {}",
            "Transcoding".bold(),
            join_humanized(targets),
            flacs.len().to_string().gray(),
            source
        );
        for target in targets {
            let jobs = self.transcode_job_factory.create(&flacs, source, *target)?;
            self.runner.add(jobs);
        }
        self.runner.execute().await?;
        info!("{} {}", "Transcoded".bold(), source);
        Ok(())
    }

    pub async fn execute_additional(
        &self,
        source: &Source,
        targets: &Vec<TargetFormat>,
    ) -> Result<(), AppError> {
        let files = Collector::get_additional(&source.directory);
        debug!(
            "{} {} additional files",
            "Adding".bold(),
            files.len().to_string().gray()
        );
        for target in targets {
            let jobs = self.additional_job_factory.create(&files, source, *target);
            self.runner.add_without_publish(jobs);
        }
        self.runner.execute_without_publish().await?;
        debug!("{} additional files {}", "Added".bold(), source);
        Ok(())
    }

    pub async fn execute_torrent(
        &self,
        source: &Source,
        targets: &Vec<TargetFormat>,
    ) -> Result<(), AppError> {
        debug!("{} torrents {}", "Creating".bold(), source);
        for target in targets {
            let content_dir = self.paths.get_transcode_target_dir(source, target);
            let output_path = self.paths.get_torrent_path(source, target);
            let announce_url = self.get_announce_url();
            let indexer = self.shared_options.get_value(|x| x.indexer.clone());
            ImdlCommand::create(&content_dir, &output_path, announce_url, indexer).await?;
            trace!("{} torrent {:?}", "Created".bold(), output_path);
        }
        debug!("{} torrents {}", "Created".bold(), source);
        Ok(())
    }

    fn get_announce_url(&self) -> String {
        let tracker_url = self.shared_options.get_value(|x| x.tracker_url.clone());
        let api_key = self.shared_options.get_value(|x| x.api_key.clone());
        format!("{tracker_url}/{api_key}/announce")
    }
}

fn join_humanized<T:Display>(strings: &[T]) -> String {
    let count = strings.len();
    if count == 0 {
        String::new()
    } else if count == 1 {
        format!("{}", strings.first().expect("should be 1"))
    } else {
        let last = strings.last().expect("should be at least 2");
        let separated : Vec<String> = strings
            .iter()
            .take(count - 1)
            .map(|x|format!("{x}"))
            .collect();
        format!("{} and {last}", separated.join(", "))
    }
}
