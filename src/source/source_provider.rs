use crate::api::{Api, Torrent};
use crate::errors::AppError;
use crate::formats::ExistingFormatProvider;
use crate::options::SharedOptions;
use crate::source::*;
use colored::Colorize;
use di::{injectable, Ref, RefMut};
use html_escape::decode_html_entities;
use log::{trace, warn};
use std::path::PathBuf;

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
        let directory = self.get_source_directory(&torrent)?;
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

    fn get_source_directory(&self, torrent: &Torrent) -> Result<PathBuf, AppError> {
        let path = decode_html_entities(&torrent.file_path).to_string();
        let directories: Vec<PathBuf> = self
            .options
            .content
            .clone()
            .expect("content should be set")
            .iter()
            .map(|x| x.join(path.clone()))
            .filter(|x| x.exists() && x.is_dir())
            .collect();
        if directories.is_empty() {
            return AppError::explained(
                "find source directory",
                "directory does not exist".to_owned(),
            );
        } else if directories.len() > 1 {
            warn!(
                "{} multiple content directories matching the torrent. The first will be used.",
                "Found".bold()
            );
            trace!("{directories:?}");
        }
        Ok(directories.first().expect("should be at least one").clone())
    }

    pub async fn get_from_options(&mut self) -> Result<Source, AppError> {
        let id = self.id_provider.get_by_options().await?;
        self.get(id).await
    }
}
