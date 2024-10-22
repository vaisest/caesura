use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use clap::Args;
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use crate::options::{DoesNotExist, OptionRule, Options, OptionsProvider};

/// Options for [`BatchCommand`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct QueueOptions {
    /// Path to queue file.
    ///
    /// Default: `output/queue.yml`
    #[arg(long)]
    pub queue: Option<PathBuf>,
}

#[injectable]
impl QueueOptions {
    fn new(provider: Ref<OptionsProvider>) -> Self {
        provider.get()
    }
}

impl Options for QueueOptions {
    fn get_name() -> String {
        "Batch Options".to_owned()
    }

    fn merge(&mut self, alternative: &Self) {
        if self.queue.is_none() {
            self.queue.clone_from(&alternative.queue);
        }
    }

    fn apply_defaults(&mut self) {
        if self.queue.is_none() {
            self.queue = Some(PathBuf::from("output/queue.yml"));
        }
    }

    #[must_use]
    fn validate(&self) -> bool {
        let mut errors: Vec<OptionRule> = Vec::new();
        if let Some(queue) = &self.queue {
            if !queue.exists() || !queue.is_file() {
                errors.push(DoesNotExist(
                    "Queue File".to_owned(),
                    queue.to_string_lossy().to_string(),
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
            Some(Queue { queue, .. }) => queue,
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
