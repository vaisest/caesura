use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use clap::Args;
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use crate::cli::QueueCommandArguments::{Add, List, Summary};
use crate::options::{Changed, DoesNotExist, OptionRule, Options, OptionsProvider};

/// Options for [`Queue`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct CacheOptions {
    /// Path to cache file.
    ///
    /// Default: `output/cache.yml`
    #[arg(long)]
    pub cache: Option<PathBuf>,
}

#[injectable]
impl CacheOptions {
    fn new(provider: Ref<OptionsProvider>) -> Self {
        provider.get()
    }
}

impl Options for CacheOptions {
    fn get_name() -> String {
        "Cache Options".to_owned()
    }

    fn merge(&mut self, alternative: &Self) {
        if self.cache.is_none() {
            self.cache.clone_from(&alternative.cache);
        }
    }

    fn apply_defaults(&mut self) {
        if self.cache.is_none() {
            self.cache = Some(PathBuf::from("output/cache.yml"));
        }
    }

    #[must_use]
    fn validate(&self) -> bool {
        let mut errors: Vec<OptionRule> = Vec::new();
        if let Some(cache) = &self.cache {
            if cache.ends_with(".json")
                || (cache.eq(&PathBuf::from("output/cache.yml")) && !cache.exists())
            {
                errors.push(Changed(
                    "Cache File".to_owned(),
                    cache.to_string_lossy().to_string(),
                    "In v0.19.0 the cache file format changed to YAML.
Please see the release notes for more details:
https://github.com/RogueOneEcho/caesura/releases/tag/v0.19.0"
                        .to_owned(),
                ));
            }

            if !cache.exists() || !cache.is_file() {
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
            Some(Batch { cache, .. }) => cache,
            Some(Queue { command, .. }) => match command {
                Add { cache, .. } => cache,
                List { cache, .. } => cache,
                Summary { cache, .. } => cache,
            },
            _ => return None,
        };
        Some(options)
    }

    fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }

    fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

impl Display for CacheOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(yaml) = serde_yaml::to_string(self) {
            yaml
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
