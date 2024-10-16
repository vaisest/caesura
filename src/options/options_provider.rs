use std::fs::read_to_string;
use std::path::PathBuf;

use colored::Colorize;
use di::injectable;
use log::*;

use crate::logging::Logger;
use crate::options::*;

const DEFAULT_CONFIG_PATH: &str = "config.json";

/// Retrieve options
///
/// Options are retrieved from multiple sources, and merged in order of precedence:
/// 1. Command line arguments
/// 2. Config file defined by the command line argument
/// 3. `config.json` in the current working directory
pub struct OptionsProvider {
    json: Option<String>,
}

#[injectable]
impl OptionsProvider {
    #[must_use]
    pub fn new() -> Self {
        let cli_options = SharedOptions::from_args().unwrap_or_default();
        Self {
            json: Some(read_config_file(&cli_options)),
        }
    }

    /// Get the [`Options`]
    #[must_use]
    pub fn get<T: Options>(&self) -> T {
        let mut options = if let Some(options) = T::from_args() {
            trace!(
                "{} {} from command line:\n{}",
                "Parsed".bold(),
                T::get_name(),
                options
            );
            options
        } else {
            T::default()
        };
        if let Some(json) = &self.json {
            if !json.is_empty() {
                match T::from_json(json) {
                    Ok(file_options) => {
                        trace!(
                            "{} {} from file:\n{}",
                            "Parsed".bold(),
                            T::get_name(),
                            file_options
                        );
                        options.merge(&file_options);
                    }
                    Err(error) => {
                        Logger::force_init();
                        error!("{} to deserialize config file: {}", "Failed".bold(), error);
                    }
                }
            }
        }
        options.apply_defaults();
        trace!("{} {}: {}", "Using".bold(), T::get_name(), options);
        options
    }
}

/// Read the config file
///
/// Use the default config path if no path is set on the command line.
fn read_config_file(options: &SharedOptions) -> String {
    let path = options
        .config
        .clone()
        .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));
    trace!("{} options from file: {:?}", "Reading".bold(), path);
    read_to_string(path).unwrap_or_else(|error| {
        Logger::force_init();
        warn!("{} to read config file: {}", "Failed".bold(), error);
        "{}".to_owned()
    })
}
