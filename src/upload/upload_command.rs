use std::path::Path;

use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{debug, error, info, trace, warn};
use tokio::fs::{copy, hard_link};

use crate::api::{Api, UploadForm};
use crate::built_info::*;
use crate::errors::AppError;
use crate::formats::{TargetFormat, TargetFormatProvider};
use crate::fs::{copy_dir, Collector, PathManager};
use crate::imdl::ImdlCommand;
use crate::jobs::Job;
use crate::options::{Options, SharedOptions, UploadOptions};
use crate::source::{get_permalink, Source, SourceProvider};
use crate::transcode::{CommandFactory, TranscodeJobFactory};

const MUSIC_CATEGORY_ID: u8 = 0;

/// Upload transcodes of a FLAC source.
#[injectable]
pub struct UploadCommand {
    shared_options: Ref<SharedOptions>,
    upload_options: Ref<UploadOptions>,
    source_provider: RefMut<SourceProvider>,
    api: RefMut<Api>,
    paths: Ref<PathManager>,
    targets: Ref<TargetFormatProvider>,
    transcode_job_factory: Ref<TranscodeJobFactory>,
}

impl UploadCommand {
    pub async fn execute(&mut self) -> Result<bool, AppError> {
        if !self.shared_options.validate() || !self.upload_options.validate() {
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

    pub async fn execute_internal(&mut self, source: &Source) -> Result<bool, AppError> {
        let targets = self.targets.get(source.format, &source.existing);
        let mut api = self.api.write().expect("API should be available to read");
        for target in targets {
            let torrent_path = self.paths.get_torrent_path(source, target);
            if !torrent_path.exists() {
                return AppError::explained(
                    "upload",
                    format!("The torrent file does not exist: {torrent_path:?}"),
                );
            }
            let target_dir = self.paths.get_transcode_target_dir(source, target);
            if let Some(error) = ImdlCommand::verify(&torrent_path, &target_dir).await? {
                error!("{} to verify the torrent content:", "Failed".bold());
                error!("{error}");
            }
            if self
                .upload_options
                .copy_transcode_to_content_dir
                .expect("copy_transcode_to_content_dir should be set")
            {
                self.copy_transcode(source, &target).await?;
            }
            if let Some(target_dir) = &self.upload_options.copy_torrent_to {
                self.copy_torrent(source, &target, target_dir).await?;
            }
            let form = UploadForm {
                path: torrent_path,
                category_id: MUSIC_CATEGORY_ID,
                remaster_year: source.metadata.year,
                remaster_title: source.torrent.remaster_title.clone(),
                remaster_record_label: source.torrent.remaster_record_label.clone(),
                remaster_catalogue_number: source.torrent.remaster_catalogue_number.clone(),
                format: target.get_file_extension().to_uppercase(),
                bitrate: target.get_bitrate().to_owned(),
                media: source.torrent.media.clone(),
                release_desc: self.create_description(source, target)?,
                group_id: source.group.id,
            };
            if self.upload_options.dry_run.expect("dry_run should be set") {
                warn!("{} upload as this is a dry run", "Skipping".bold());
                info!("{} data of {target} for {source}:", "Upload".bold());
                info!("{}", form);
                continue;
            }
            let response = api.upload_torrent(form).await?;
            debug!("{} {target} for {source}", "Uploaded".bold());
            let base = &self
                .shared_options
                .indexer_url
                .clone()
                .expect("indexer_url should be set");
            debug!(
                "{}",
                get_permalink(base, response.group_id, response.torrent_id)
            );
        }
        info!("{} {source}", "Uploaded".bold());
        Ok(true)
    }

    async fn copy_transcode(&self, source: &Source, target: &TargetFormat) -> Result<(), AppError> {
        let source_dir = self.paths.get_transcode_target_dir(source, *target);
        let source_dir_name = source_dir
            .file_name()
            .expect("source dir should have a name");
        let target_dir = self
            .shared_options
            .content
            .clone()
            .expect("content should be set")
            .join(source_dir_name);
        let verb = if self
            .upload_options
            .hard_link
            .expect("hard_link should be set")
        {
            copy_dir(&source_dir, &target_dir, true).await?;
            "Hard Linked"
        } else {
            copy_dir(&source_dir, &target_dir, false).await?;
            "Copied"
        };
        trace!("{} {source_dir:?} to {target_dir:?}", verb.bold());
        Ok(())
    }

    async fn copy_torrent(
        &self,
        source: &Source,
        target: &TargetFormat,
        target_dir: &Path,
    ) -> Result<(), AppError> {
        let source_path = self.paths.get_torrent_path(source, *target);
        let source_file_name = source_path
            .file_name()
            .expect("torrent path should have a name");
        let target_path = target_dir.join(source_file_name);
        let verb = if self
            .upload_options
            .hard_link
            .expect("hard_link should be set")
        {
            hard_link(&source_path, &target_path)
                .await
                .or_else(|e| AppError::io(e, "hard link torrent file"))?;
            "Hard Linked"
        } else {
            copy(&source_path, &target_path)
                .await
                .or_else(|e| AppError::io(e, "copy torrent file"))?;
            "Copied"
        };
        trace!("{} {source_path:?} to {target_path:?}", verb.bold());
        Ok(())
    }

    #[allow(clippy::uninlined_format_args)]
    fn create_description(
        &self,
        source: &Source,
        target: TargetFormat,
    ) -> Result<String, AppError> {
        let base = &self
            .shared_options
            .indexer_url
            .clone()
            .expect("indexer_url should be set");
        let source_url = get_permalink(base, source.group.id, source.torrent.id);
        let source_title = source.format.get_title();
        let transcode_command = self.get_command(source, target)?;
        let lines: Vec<String> = [
            format!(
                "Transcoded and uploaded with [url={}][b]{}[/b] v{}[/url]",
                PKG_REPOSITORY, PKG_NAME, PKG_VERSION
            ),
            format!("[pad=0|10|0|20]Source[/pad] [url={source_url}]{source_title}[/url]"),
            format!("[pad=0|10|0|0]Command[/pad] [code]{transcode_command}[/code]"),
            format!(
                "[url={}]Learn how easy it is to create and upload transcodes yourself![/url]",
                PKG_REPOSITORY
            ),
        ]
        .iter()
        .map(|line| format!("[quote]{line}[/quote]"))
        .collect();
        let description = lines.join("");
        Ok(description)
    }

    pub fn get_command(&self, source: &Source, target: TargetFormat) -> Result<String, AppError> {
        let flacs = Collector::get_flacs(&source.directory);
        let flac = flacs.first().expect("Should be at least one FLAC");
        let job = self
            .transcode_job_factory
            .create_single(0, flac, source, target)?;

        let Job::Transcode(job) = job else {
            return AppError::explained(
                "get transcode command",
                "expected a transcode job".to_owned(),
            );
        };
        let commands: Vec<String> = job
            .commands
            .iter()
            .map(CommandFactory::to_cli_string)
            .collect();
        let command = commands.join(" | ");
        let input_path = flac.path.to_string_lossy().to_string();
        let output_path = job.output_path.to_string_lossy().to_string();
        let output_extension = job
            .output_path
            .extension()
            .expect("ouput path should have an extension")
            .to_string_lossy()
            .to_string();
        let command = command.replace(input_path.as_str(), "input.flac").replace(
            output_path.as_str(),
            format!("output.{output_extension}").as_str(),
        );
        Ok(command)
    }
}
