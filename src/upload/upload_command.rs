use std::path::Path;

use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{debug, error, info, trace};
use tokio::fs::{copy, hard_link};

use crate::api::{Api, UploadForm};
use crate::built_info::*;
use crate::errors::AppError;
use crate::formats::{TargetFormat, TargetFormatProvider};
use crate::fs::{Collector, PathManager};
use crate::imdl::ImdlCommand;
use crate::jobs::Job;
use crate::options::{Options, SharedOptions, UploadOptions};
use crate::source::{get_permalink, Source};
use crate::transcode::TranscodeJobFactory;

/// Upload transcodes of a FLAC source.
#[injectable]
pub struct UploadCommand {
    options: Ref<SharedOptions>,
    upload_options: Ref<UploadOptions>,
    api: RefMut<Api>,
    paths: Ref<PathManager>,
    targets: Ref<TargetFormatProvider>,
    transcode_job_factory: Ref<TranscodeJobFactory>
}

impl UploadCommand {
    pub async fn execute(&mut self, source: &Source) -> Result<bool, AppError> {
        let targets = self.targets.get(source.format, &source.existing);
        let mut api = self.api.write().expect("API should be available to read");
        for target in targets {
            let torrent_path = self.paths.get_torrent_path(source, &target);
            if !torrent_path.exists() {
                return AppError::explained(
                    "upload",
                    format!("The torrent file does not exist: {torrent_path:?}"),
                );
            }
            let target_dir = self.paths.get_transcode_target_dir(source, &target);
            if let Some(error) = ImdlCommand::verify(&torrent_path, &target_dir).await? {
                error!("{} to verify the torrent content:", "Failed".bold().red(),);
                error!("{error}");
            }
            if self
                .upload_options
                .get_value(|x| x.copy_transcode_to_content_dir)
            {
                self.copy_transcode(source, &target).await?;
            }
            if let Some(target_dir) = &self.upload_options.copy_torrent_to {
                self.copy_torrent(source, &target, target_dir).await?;
            }
            let form = UploadForm {
                path: torrent_path,
                category_id: source.group.category_name.as_int(),
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
            let response = api.upload_torrent(form).await?;
            debug!("{} {target} for {source}", "Uploaded".bold());
            let base = &self.options.get_value(|x| x.indexer_url.clone());
            debug!("{}", get_permalink(base, response.group_id, response.torrent_id));
        }
        info!("{} {source}", "Uploaded".bold());
        Ok(true)
    }

    async fn copy_transcode(&self, source: &Source, target: &TargetFormat) -> Result<(), AppError> {
        let source_dir = self.paths.get_transcode_target_dir(source, target);
        let source_dir_name = source_dir
            .file_name()
            .expect("source dir should have a name");
        let target_dir = self
            .options
            .get_value(|x| x.content_directory.clone())
            .join(source_dir_name);
        let verb = if self.upload_options.get_value(|x| x.hard_link) {
            hard_link(&source_dir, &target_dir)
                .await
                .or_else(|e| AppError::io(e, "hard link transcode content"))?;
            "Hard Linked"
        } else {
            copy(&source_dir, &target_dir)
                .await
                .or_else(|e| AppError::io(e, "copy transcode content"))?;
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
        let source_path = self.paths.get_torrent_path(source, target);
        let source_file_name = source_path
            .file_name()
            .expect("torrent path should have a name");
        let target_path = target_dir.join(source_file_name);
        let verb = if self.upload_options.get_value(|x| x.hard_link) {
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
    fn create_description(&self, source: &Source, target: TargetFormat) -> Result<String, AppError> {
        let base = &self.options.get_value(|x| x.indexer_url.clone());
        let source_url = get_permalink(base, source.group.id, source.torrent.id);
        let transcode_command = self.get_command(source, target)?;
        Ok(format!(
            "Transcode of [url]{source_url}[/url]\n\
            Transcode process:\n\
            [code]{transcode_command}[/code]\n\
            Created using [url={}]{} v{}[/url]",
            PKG_REPOSITORY, PKG_NAME, PKG_VERSION
        ))
    }

    pub fn get_command(&self, source: &Source, target: TargetFormat) -> Result<String, AppError> {
        let flacs = Collector::get_flacs(&source.directory);
        let flac = flacs.first().expect("Should be at least one FLAC");
        let job = self.transcode_job_factory.create_single(0, flac, source, target)?;
        let job = match job {
            Job::Transcode(transcode_job) => transcode_job,
            _ => return AppError::explained("get transcode command", "".to_owned()),
        };
        let commands : Vec<String> = job
            .commands
            .iter()
            .map(|command| command.to_cli_string())
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
        let command = command
            .replace(input_path.as_str(), "input.flac")
            .replace(output_path.as_str(), format!("output.{output_extension}").as_str());
        Ok(command)
    }
}
