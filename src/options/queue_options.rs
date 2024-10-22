use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::cli::CommandArguments::Queue;
use crate::cli::{ArgumentsParser, QueueCommandArguments};
use crate::options::{DoesNotExist, NotSet, OptionRule, Options, OptionsProvider};
use clap::Args;
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};
use QueueCommandArguments::Add;

/// Options for the [`QueueCommand`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct QueueOptions {
    /// Path to directory of `.torrent` files.
    ///
    /// Examples: `./torrents`, `/path/to/torrents`
    #[arg(value_name = "TORRENT_DIRECTORY")]
    pub torrents: Option<PathBuf>,
}

#[injectable]
impl QueueOptions {
    fn new(provider: Ref<OptionsProvider>) -> Self {
        provider.get()
    }
}

impl Options for QueueOptions {
    fn get_name() -> String {
        "Shared Options".to_owned()
    }

    fn merge(&mut self, alternative: &Self) {
        if self.torrents.is_none() {
            self.torrents.clone_from(&alternative.torrents);
        }
    }

    fn apply_defaults(&mut self) {}

    #[must_use]
    fn validate(&self) -> bool {
        let mut errors: Vec<OptionRule> = Vec::new();
        if let Some(torrent_dir) = &self.torrents {
            if !torrent_dir.exists() || !torrent_dir.is_dir() {
                errors.push(DoesNotExist(
                    "Torrent Directory".to_owned(),
                    torrent_dir.to_string_lossy().to_string(),
                ));
            }
        } else {
            errors.push(NotSet("Torrent Directory".to_owned()));
        }
        OptionRule::show(&errors);
        errors.is_empty()
    }

    #[allow(clippy::match_wildcard_for_single_variants)]
    fn from_args() -> Option<Self> {
        match ArgumentsParser::get() {
            Some(Queue {
                command: Add { queue, .. },
                ..
            }) => Some(queue),
            _ => None,
        }
    }

    fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }

    fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

impl Display for QueueOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(yaml) = serde_yaml::to_string(self) {
            yaml
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
