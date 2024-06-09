use std::fmt::{Display, Formatter};

use clap::{ArgAction, Args};
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use crate::options::{Options, OptionsProvider, ValueProvider};

/// Options for [`BatchCommand`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct BatchOptions {
    /// Should the spectrogram command be executed?
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub no_spectrogram: Option<bool>,

    /// Should the upload command be executed?
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub no_upload: Option<bool>,
}

#[injectable]
impl BatchOptions {
    fn new(provider: Ref<OptionsProvider>) -> Self {
        provider.get()
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
    }

    fn apply_defaults(&mut self) {
        if self.no_spectrogram.is_none() {
            self.no_spectrogram = Some(false);
        }
        if self.no_upload.is_none() {
            self.no_upload = Some(false);
        }
    }

    #[must_use]
    fn validate(&self) -> bool {
        true
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
