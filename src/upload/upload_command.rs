use std::path::Path;

use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::{error, trace};
use tokio::fs::{copy, hard_link};

use crate::api::{Api, UploadForm};
use crate::built_info::*;
use crate::errors::AppError;
use crate::formats::{TargetFormat, TargetFormatProvider};
use crate::fs::PathManager;
use crate::imdl::ImdlCommand;
use crate::options::{Options, SharedOptions, UploadOptions};
use crate::source::{get_permalink, Source};

/// Upload transcodes of a FLAC source.
#[injectable]
pub struct UploadCommand {
    options: Ref<SharedOptions>,
    upload_options: Ref<UploadOptions>,
    api: RefMut<Api>,
    paths: Ref<PathManager>,
    targets: Ref<TargetFormatProvider>,
}

impl UploadCommand {
    pub async fn execute(&mut self, source: &Source) -> Result<bool, AppError> {
        // TODO MUST include transcode process
        let transcode_command = String::new();
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
                category_id: source.group.category_id,
                remaster_year: source.metadata.year,
                remaster_title: source.torrent.remaster_title.clone(),
                remaster_record_label: source.torrent.remaster_record_label.clone(),
                remaster_catalogue_number: source.torrent.remaster_catalogue_number.clone(),
                format: target.get_file_extension().to_uppercase(),
                bitrate: target.get_bitrate().to_owned(),
                media: source.torrent.media.clone(),
                release_desc: self.create_description(source, &transcode_command),
                group_id: source.group.id,
            };
            api.upload_torrent(form).await?;
        }
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
                .or_else(|e| AppError::io(e, "hard link additional file"))?;
            "Hard Linked"
        } else {
            copy(&source_dir, &target_dir)
                .await
                .or_else(|e| AppError::io(e, "copy additional file"))?;
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
                .or_else(|e| AppError::io(e, "hard link additional file"))?;
            "Hard Linked"
        } else {
            copy(&source_path, &target_path)
                .await
                .or_else(|e| AppError::io(e, "copy additional file"))?;
            "Copied"
        };
        trace!("{} {source_path:?} to {target_path:?}", verb.bold());
        Ok(())
    }

    #[allow(clippy::uninlined_format_args)]
    fn create_description(&self, source: &Source, transcode_command: &String) -> String {
        let base = &self.options.get_value(|x| x.indexer_url.clone());
        let source_url = get_permalink(base, source.group.id, source.torrent.id);
        format!(
            "Transcode of [url]{source_url}[/url]\
            Transcode process:\
            [code]{transcode_command}[/code]\
            Created using [url={}]{} v{}[/url]",
            PKG_REPOSITORY, PKG_NAME, PKG_VERSION
        )
    }
}
