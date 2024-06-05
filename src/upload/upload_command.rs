use di::{injectable, Ref, RefMut};

use crate::api::{Api, UploadForm};
use crate::built_info::*;
use crate::errors::AppError;
use crate::formats::TargetFormatProvider;
use crate::fs::PathManager;
use crate::options::SharedOptions;
use crate::source::{get_permalink, Source};

/// Upload transcodes of a FLAC source.
#[injectable]
pub struct UploadCommand {
    options: Ref<SharedOptions>,
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
            // TODO SHOULD copy files to content

            let form = UploadForm {
                path: self.paths.get_torrent_path(source, &target),
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

    fn create_description(&self, source: &Source, transcode_command: &String) -> String {
        let base = &self
            .options
            .indexer_url
            .clone()
            .expect("Options should be set");
        let source_url = get_permalink(base, source.group.id, source.torrent.id);
        return format!(
            "Transcode of [url]{source_url}[/url]\
            Transcode process:\
            [code]{transcode_command}[/code]\
            Created using [url={}]{} v{}[/url]",
            PKG_REPOSITORY, PKG_NAME, PKG_VERSION
        );
    }
}
