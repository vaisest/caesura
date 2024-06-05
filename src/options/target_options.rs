use std::fmt::{Display, Formatter};

use clap::{ArgAction, Args};
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use crate::formats::TargetFormat;
use crate::options::{IsEmpty, NotSet, OptionRule, Options, OptionsProvider, ValueProvider};

/// Options for [`TranscodeCommand`] and [`VerifyCommand`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct TargetOptions {
    /// Target formats.
    /// Default: flac, 320, and v0
    #[arg(long)]
    pub target: Option<Vec<TargetFormat>>,

    /// Allow transcoding to existing formats
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub allow_existing: Option<bool>,
}

#[injectable]
impl TargetOptions {
    fn new(provider: Ref<OptionsProvider>) -> Self {
        provider.get()
    }
}

impl Options for TargetOptions {
    fn get_name() -> String {
        "Transcode Options".to_owned()
    }

    fn get_value<TValue, F>(&self, select: F) -> TValue
    where
        F: FnOnce(&Self) -> Option<TValue>,
    {
        ValueProvider::get(self, select)
    }

    fn merge(&mut self, alternative: &Self) {
        if self.target.is_none() {
            self.target.clone_from(&alternative.target);
        }
        if self.allow_existing.is_none() {
            self.allow_existing = alternative.allow_existing;
        }
    }

    fn apply_defaults(&mut self) {
        if self.target.is_none() {
            self.target = Some(vec![
                TargetFormat::Flac,
                TargetFormat::_320,
                TargetFormat::V0,
            ]);
        }
        if self.allow_existing.is_none() {
            self.allow_existing = Some(false);
        }
    }

    #[must_use]
    fn validate(&self) -> bool {
        let mut errors: Vec<OptionRule> = Vec::new();
        if let Some(targets) = &self.target {
            if targets.is_empty() {
                errors.push(IsEmpty("Target format".to_owned()));
            }
        } else {
            errors.push(NotSet("Target format".to_owned()));
        }
        OptionRule::show(&errors);
        errors.is_empty()
    }

    #[must_use]
    fn from_args() -> Option<Self> {
        let options = match ArgumentsParser::get() {
            Some(Transcode { target, .. }) => target,
            Some(Verify { target, .. }) => target,
            Some(Upload { target, .. }) => target,
            _ => return None,
        };
        let mut options = options;
        if options.allow_existing == Some(false) {
            options.allow_existing = None;
        }
        Some(options)
    }

    fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }
}

impl Display for TargetOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(json) = serde_json::to_string_pretty(self) {
            json
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
