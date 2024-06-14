use di::{injectable, Ref, RefMut};
use html_escape::decode_html_entities;
use std::path::Path;

use crate::api::Api;
use crate::errors::AppError;
use crate::formats::ExistingFormatProvider;
use crate::options::{Options, SharedOptions};
use crate::source::*;

/// Retrieve [Source] from the [Api] via a [provider design pattern](https://en.wikipedia.org/wiki/Provider_model)
#[injectable]
pub struct SourceProvider {
    api: RefMut<Api>,
    options: Ref<SharedOptions>,
    id_provider: Ref<IdProvider>,
}

impl SourceProvider {
    pub async fn get(&mut self, id: i64) -> Result<Source, AppError> {
        let mut api = self.api.write().expect("API should be available to read");
        let response = api.get_torrent(id).await?;
        let torrent = response.torrent;
        let group = response.group;
        let response = api.get_torrent_group(group.id).await?;
        if group.id != response.group.id {
            AppError::explained(
                "get source by id",
                "group of torrent did not match torrent group".to_owned(),
            )?;
        }
        let group_torrents = response.torrents;
        let format = torrent.get_format()?.to_source()?;
        let existing = ExistingFormatProvider::get(&torrent, &group_torrents)?;
        let directory = self
            .options
            .get_value(|x| x.content_directory.clone())
            .join(decode_html_entities(&torrent.file_path).to_string());
        let metadata = Metadata::new(&group, &torrent);
        Ok(Source {
            torrent,
            group,
            existing,
            format,
            directory,
            metadata,
        })
    }

    pub async fn get_from_options(&mut self) -> Result<Source, AppError> {
        let id = self.id_provider.get_by_options().await?;
        self.get(id).await
    }

    pub async fn get_from_directory(&mut self, directory: &Path) -> Result<Vec<Source>, AppError> {
        let ids = self.id_provider.get_by_directory(directory).await?;
        let mut sources: Vec<Source> = Vec::new();
        for id in ids {
            let source = self.get(id).await?;
            sources.push(source);
        }
        Ok(sources)
    }
}
