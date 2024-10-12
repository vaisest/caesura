use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use clap::{ArgAction, Args};
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use crate::options::{DoesNotExist, OptionRule, Options, OptionsProvider, ValueProvider};

/// Options for [`BatchCommand`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct BatchOptions {
    /// Should the spectrogram command be executed?
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub no_spectrogram: Option<bool>,

    /// Should the upload command be executed?
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub no_upload: Option<bool>,

    /// Limit the number of torrents to batch process.
    #[arg(long)]
    pub limit: Option<usize>,

    /// Wait for a duration before uploading the torrent.
    ///
    /// The duration is a string that can be parsed such as `500ms`, `5m`, `1h30m15s`.
    #[arg(long)]
    pub wait_before_upload: Option<String>,

    /// Path to cache file.
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
}

impl Options for BatchOptions {
    fn get_name() -> String {
        "Batch Options".to_owned()
    }

    fn get_value<TValue, F>(&self, select: F) -> TValue
    where
        F: FnOnce(&Self) -> Option<TValue>,
    {
        ValueProvider::get(self, select)
    }

    fn merge(&mut self, alternative: &Self) {
        if self.no_spectrogram.is_none() {
            self.no_spectrogram = alternative.no_spectrogram;
        }
        if self.no_upload.is_none() {
            self.no_upload = alternative.no_upload;
        }
        if self.limit.is_none() {
            self.limit = alternative.limit;
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
        if self.no_spectrogram.is_none() {
            self.no_spectrogram = Some(false);
        }
        if self.no_upload.is_none() {
            self.no_upload = Some(false);
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
        if options.no_spectrogram == Some(false) {
            options.no_spectrogram = None;
        }
        if options.no_upload == Some(false) {
            options.no_upload = None;
        }
        Some(options)
    }

    fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }
}

impl Display for BatchOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(json) = serde_json::to_string_pretty(self) {
            json
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
