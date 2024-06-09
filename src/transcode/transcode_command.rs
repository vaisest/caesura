use std::fs::create_dir_all;

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
            .get()
            .await?;
        self.execute_internal(&source).await
    }

    pub async fn execute_internal(&self, source: &Source) -> Result<bool, AppError> {
        let targets = self.targets.get(source.format, &source.existing);
        let output_dir = self.paths.get_transcode_dir(source);
        self.execute_transcode(source, &targets).await?;
        self.execute_additional(source, &targets).await?;
        self.execute_torrent(source, &targets).await?;
        debug!(
            "{} {}",
            "in".gray(),
            output_dir.to_string_lossy().to_string().gray()
        );
        Ok(true)
    }

    pub async fn execute_transcode(
        &self,
        source: &Source,
        targets: &Vec<TargetFormat>,
    ) -> Result<(), AppError> {
        let flacs = Collector::get_flacs(&source.directory);
        info!(
            "{} to {:?} for {} FLACs in {}",
            "Transcoding".bold(),
            targets,
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
            self.runner.add(jobs);
        }
        self.runner.execute().await?;
        debug!("{} additional files {}", "Added".bold(), source);
        Ok(())
    }

    pub async fn execute_torrent(
        &self,
        source: &Source,
        targets: &Vec<TargetFormat>,
    ) -> Result<(), AppError> {
        debug!("{} torrents {}", "Creating".bold(), source);
        let torrent_dir = self.paths.get_torrent_dir(source);
        create_dir_all(&torrent_dir)
            .or_else(|e| AppError::io(e, "create torrent output directory"))?;
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
