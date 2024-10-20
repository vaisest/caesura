use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use clap::{ArgAction, Args};
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use crate::options::{DoesNotExist, OptionRule, Options, OptionsProvider};

/// Options for [`BatchCommand`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct BatchOptions {
    /// Should the spectrogram command be executed?
    ///
    /// Default: `false`
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub spectrogram: Option<bool>,

    /// Should the upload command be executed?
    ///
    /// Default: `false`
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub upload: Option<bool>,

    /// Limit the number of torrents to batch process.
    ///
    /// If `no_limit` is set, this option is ignored.
    ///
    /// Default: `3`
    #[arg(long)]
    pub limit: Option<usize>,

    /// Should the `limit` option be ignored?
    ///
    /// Default: `false`
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub no_limit: Option<bool>,

    /// Wait for a duration before uploading the torrent.
    ///
    /// The duration is a string that can be parsed such as `500ms`, `5m`, `1h30m15s`.
    ///
    /// Default: `null`
    #[arg(long)]
    pub wait_before_upload: Option<String>,

    /// Path to cache file.
    ///
    /// Default: `output/cache.json`
    #[arg(long)]
    pub cache: Option<PathBuf>,
}

#[injectable]
impl BatchOptions {
    fn new(provider: Ref<OptionsProvider>) -> Self {
        provider.get()
    }

    #[must_use]
    pub fn get_wait_before_upload(&self) -> Option<std::time::Duration> {
        let wait_before_upload = self.wait_before_upload.clone()?;
        humantime::parse_duration(wait_before_upload.as_str()).ok()
    }

    #[must_use]
    pub fn get_limit(&self) -> Option<usize> {
        if self.no_limit == Some(true) {
            None
        } else {
            self.limit
        }
    }
}

impl Options for BatchOptions {
    fn get_name() -> String {
        "Batch Options".to_owned()
    }

    fn merge(&mut self, alternative: &Self) {
        if self.spectrogram.is_none() {
            self.spectrogram = alternative.spectrogram;
        }
        if self.upload.is_none() {
            self.upload = alternative.upload;
        }
        if self.limit.is_none() {
            self.limit = alternative.limit;
        }
        if self.no_limit.is_none() {
            self.no_limit = alternative.no_limit;
        }
        if self.wait_before_upload.is_none() {
            self.wait_before_upload
                .clone_from(&alternative.wait_before_upload);
        }
        if self.cache.is_none() {
            self.cache.clone_from(&alternative.cache);
        }
    }

    fn apply_defaults(&mut self) {
        if self.spectrogram.is_none() {
            self.spectrogram = Some(false);
        }
        if self.upload.is_none() {
            self.upload = Some(false);
        }
        if self.limit.is_none() {
            self.limit = Some(3);
        }
        if self.no_limit.is_none() {
            self.no_limit = Some(false);
        }
        if self.cache.is_none() {
            self.cache = Some(PathBuf::from("output/cache.json"));
        }
    }

    #[must_use]
    fn validate(&self) -> bool {
        let mut errors: Vec<OptionRule> = Vec::new();
        if let Some(wait_before_upload) = &self.wait_before_upload {
            if self.get_wait_before_upload().is_none() {
                errors.push(OptionRule::DurationInvalid(
                    "Wait Before Upload".to_owned(),
                    wait_before_upload.clone(),
                ));
            }
        }
        if let Some(cache) = &self.cache {
            if !cache.exists() && !cache.is_file() {
                errors.push(DoesNotExist(
                    "Cache File".to_owned(),
                    cache.to_string_lossy().to_string(),
                ));
            }
        }
        OptionRule::show(&errors);
        errors.is_empty()
    }

    #[must_use]
    #[allow(clippy::manual_let_else)]
    fn from_args() -> Option<Self> {
        let options = match ArgumentsParser::get() {
            Some(Batch { batch, .. }) => batch,
            _ => return None,
        };
        let mut options = options;
        if options.spectrogram == Some(false) {
            options.spectrogram = None;
        }
        if options.upload == Some(false) {
            options.upload = None;
        }
        Some(options)
    }

    fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }

    fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

impl Display for BatchOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(yaml) = serde_yaml::to_string(self) {
            yaml
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
