use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::*;
use std::collections::BTreeSet;

use crate::errors::AppError;
use crate::formats::{TargetFormat, TargetFormatProvider};
use crate::fs::{Collector, PathManager};
use crate::imdl::ImdlCommand;
use crate::jobs::JobRunner;
use crate::logging::Colors;
use crate::naming::join_humanized;
use crate::options::{Options, SharedOptions, TargetOptions};
use crate::queue::TimeStamp;
use crate::source::*;
use crate::transcode::{
    AdditionalJobFactory, TranscodeFormatStatus, TranscodeJobFactory, TranscodeStatus,
};

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
    /// Execute [`TranscodeCommand`] from the CLI.
    ///
    /// [`Source`] is retrieved from the CLI arguments.
    ///
    /// Returns `true` if all the transcodes succeeds.
    pub async fn execute_cli(&self) -> Result<bool, AppError> {
        if !self.shared_options.validate() || !self.target_options.validate() {
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

    /// Execute [`TranscodeCommand`] on a [`Source`].
    ///
    /// Returns a [`TranscodeStatus`] indicating the success of the operation and any errors.
    ///
    /// Errors are not logged so should be handled by the caller.
    #[must_use]
    pub async fn execute(&self, source: &Source) -> TranscodeStatus {
        let targets = self.targets.get(source.format, &source.existing);
        let targets = self.skip_completed(source, &targets);
        let mut status = TranscodeStatus {
            success: false,
            formats: None,
            additional: None,
            completed: TimeStamp::now(),
            error: None,
        };
        if targets.is_empty() {
            status.error = Some(AppError::else_explained(
                "transcode",
                "No transcodes to perform".to_owned(),
            ));
            return status;
        }
        if let Err(error) = self.execute_transcode(source, &targets).await {
            status.error = Some(error);
            status.completed = TimeStamp::now();
            return status;
        }
        let formats: Vec<TranscodeFormatStatus> = targets
            .iter()
            .map(|&format| TranscodeFormatStatus {
                format,
                path: self.paths.get_transcode_target_dir(source, format),
            })
            .collect();
        status.formats = Some(formats);
        if let Err(error) = self.execute_additional(source, &targets).await {
            status.error = Some(error);
            status.completed = TimeStamp::now();
            return status;
        }
        if let Err(error) = self.execute_torrent(source, &targets).await {
            status.error = Some(error);
            status.completed = TimeStamp::now();
            return status;
        }
        status.success = true;
        status
    }

    #[must_use]
    fn skip_completed(
        &self,
        source: &Source,
        targets: &BTreeSet<TargetFormat>,
    ) -> BTreeSet<TargetFormat> {
        let mut out: BTreeSet<TargetFormat> = BTreeSet::new();
        for target in targets {
            let path = self.paths.get_torrent_path(source, *target);
            if path.exists() {
                debug!("{} existing {target} transcode", "Found".bold());
                trace!("{path:?}");
            } else {
                out.insert(*target);
            }
        }
        out
    }

    async fn execute_transcode(
        &self,
        source: &Source,
        targets: &BTreeSet<TargetFormat>,
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

    async fn execute_additional(
        &self,
        source: &Source,
        targets: &BTreeSet<TargetFormat>,
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

    async fn execute_torrent(
        &self,
        source: &Source,
        targets: &BTreeSet<TargetFormat>,
    ) -> Result<(), AppError> {
        debug!("{} torrents {}", "Creating".bold(), source);
        for target in targets {
            let content_dir = self.paths.get_transcode_target_dir(source, *target);
            let output_path = self.paths.get_torrent_path(source, *target);
            let announce_url = self
                .shared_options
                .announce_url
                .clone()
                .expect("announce_url should be set");
            let indexer = self
                .shared_options
                .indexer
                .clone()
                .expect("indexer should be set");
            ImdlCommand::create(&content_dir, &output_path, announce_url, indexer).await?;
            trace!("{} torrent {:?}", "Created".bold(), output_path);
        }
        debug!("{} torrents {}", "Created".bold(), source);
        Ok(())
    }
}
