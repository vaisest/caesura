use std::path::{Path, PathBuf};

use colored::Colorize;
use di::{injectable, Ref};
use log::debug;

use crate::errors::{AppError, Reason};
use crate::fs::DirectoryReader;
use crate::imdl::ImdlCommand;
use crate::options::{Options, SharedOptions};
use crate::source::*;

/// Retrieve the id of a source.
#[injectable]
pub struct IdProvider {
    options: Ref<SharedOptions>,
}

impl IdProvider {
    pub async fn get_by_options(&self) -> Result<i64, AppError> {
        let source_input = self.options.source.clone().unwrap_or_default();
        self.get_by_string(&source_input).await
    }

    pub async fn get_by_string(&self, input: &String) -> Result<i64, AppError> {
        let id = if let Ok(id) = input.parse::<i64>() {
            id
        } else if input.starts_with("http") {
            self.get_by_url(input)?
        } else if input.ends_with(".torrent") {
            let path = PathBuf::from(input);
            if path.exists() {
                self.get_by_file(&path).await?
            } else {
                AppError::explained(
                    "get source from torrent file",
                    "File does not exist".to_owned(),
                )?
            }
        } else {
            AppError::explained("get source", format!("Unknown source: {input}"))?
        };
        Ok(id)
    }

    fn get_by_url(&self, url: &str) -> Result<i64, AppError> {
        let base = &self.options.get_value(|x| x.indexer_url.clone());
        get_torrent_id_from_url(url, base)
    }

    async fn get_by_file(&self, path: &Path) -> Result<i64, AppError> {
        let summary = ImdlCommand::show(path).await?;
        let tracker_id = self.options.get_value(|x| x.indexer.clone()).to_uppercase();
        if summary.source == Some(tracker_id.clone()) {
            let url = summary.comment.unwrap_or_default();
            self.get_by_url(&url)
        } else {
            AppError::unexpected(
                "get source by file",
                "incorrect source",
                tracker_id,
                summary.source.unwrap_or_default(),
            )
        }
    }

    pub async fn get_by_directory(&self, directory: &Path) -> Result<Vec<i64>, AppError> {
        if !directory.is_dir() {
            return AppError::explained(
                "get source by directory",
                "path is not a directory".to_owned(),
            );
        }
        let paths = DirectoryReader::new()
            .with_extension("torrent")
            .with_max_depth(0)
            .read(directory)
            .or_else(|e| AppError::io(e, "read source directory"))?;
        let mut ids: Vec<i64> = Vec::new();
        for path in paths {
            match self.get_by_file(&path).await {
                Ok(id) => ids.push(id),
                Err(error) => {
                    let explanation = match error.reason {
                        Reason::Explained(explanation) => explanation,
                        Reason::Unexpected(explanation, _, _) => explanation,
                        Reason::External(_, _) => "unknown reason".to_owned(),
                    };
                    debug!("{} {explanation} {path:?}", "Skipped".bold());
                }
            }
        }
        Ok(ids)
    }
}
