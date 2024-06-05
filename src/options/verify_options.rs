use std::fmt::{Display, Formatter};

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use clap::{ArgAction, Args};
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};

use crate::options::{Options, OptionsProvider};

/// Options for [Verifyr] and [`SourceVerifier`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct VerifyOptions {
    /// Should the torrent hash check of existing files be skipped?
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub skip_hash_check: Option<bool>,
}

#[injectable]
impl VerifyOptions {
    fn new(provider: Ref<OptionsProvider>) -> Self {
        provider.get()
    }
}

impl Options for VerifyOptions {
    fn get_name() -> String {
        "Verify Options".to_owned()
    }

    /// Merge the current options with an alternative set of options
    ///
    /// The current options will take precedence over the alternative options
    fn merge(&mut self, alternative: &Self) {
        if self.skip_hash_check.is_none() {
            self.skip_hash_check = alternative.skip_hash_check;
        }
    }

    fn apply_defaults(&mut self) {
        if self.skip_hash_check.is_none() {
            self.skip_hash_check = Some(false);
        }
    }

    /// Validate the options
    #[must_use]
    fn validate(&self) -> bool {
        true
    }

    #[must_use]
    fn from_args() -> Option<Self> {
        let options = match ArgumentsParser::get() {
            Some(Verify { verify, .. }) => verify,
            _ => return None,
        };
        let mut options = options;
        if options.skip_hash_check == Some(false) {
            options.skip_hash_check = None;
        }
        Some(options)
    }

    fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }
}

impl Display for VerifyOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(json) = serde_json::to_string_pretty(self) {
            json
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
