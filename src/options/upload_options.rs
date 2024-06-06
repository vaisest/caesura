use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use clap::{ArgAction, Args};
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use crate::options::{Options, OptionsProvider, ValueProvider};

/// Options for including additional files during [`TranscodeCommand`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct UploadOptions {
    /// Should the transcoded files be copied to the content directory.
    ///
    /// This should be enabled if you wish to auto-add to your torrent client.
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub copy_transcode_to_content_dir: Option<bool>,

    /// Copy the torrent file to the provided directory.
    ///
    /// This should be set if you wish to auto-add to your torrent client.
    #[arg(long)]
    pub copy_torrent_to: Option<PathBuf>,

    /// Use hard links when copying files.
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub hard_link: Option<bool>,
}

#[injectable]
impl UploadOptions {
    fn new(provider: Ref<OptionsProvider>) -> Self {
        provider.get()
    }
}

impl Options for UploadOptions {
    fn get_name() -> String {
        "Upload Options".to_owned()
    }

    fn get_value<TValue, F>(&self, select: F) -> TValue
    where
        F: FnOnce(&Self) -> Option<TValue>,
    {
        ValueProvider::get(self, select)
    }

    fn merge(&mut self, alternative: &Self) {
        if self.copy_transcode_to_content_dir.is_none() {
            self.copy_transcode_to_content_dir = alternative.copy_transcode_to_content_dir;
        }
        if self.copy_torrent_to.is_none() {
            self.copy_torrent_to
                .clone_from(&alternative.copy_torrent_to);
        }
        if self.hard_link.is_none() {
            self.hard_link = alternative.hard_link;
        }
    }

    fn apply_defaults(&mut self) {
        if self.copy_transcode_to_content_dir.is_none() {
            self.copy_transcode_to_content_dir = Some(false);
        }
        if self.hard_link.is_none() {
            self.hard_link = Some(false);
        }
    }

    #[must_use]
    fn validate(&self) -> bool {
        true
    }

    #[must_use]
    fn from_args() -> Option<Self> {
        let Some(Upload {
            upload: options, ..
        }) = ArgumentsParser::get()
        else {
            return None;
        };
        let mut options = options;
        if options.copy_transcode_to_content_dir == Some(false) {
            options.copy_transcode_to_content_dir = None;
        }
        if options.hard_link == Some(false) {
            options.hard_link = None;
        }
        Some(options)
    }

    fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }
}

impl Display for UploadOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(json) = serde_json::to_string_pretty(self) {
            json
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
