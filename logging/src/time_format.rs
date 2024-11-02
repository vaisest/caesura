use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum TimeFormat {
    /// Local date and time in an ISO 8601 like format.
    ///
    /// Examples: `2013-02-27 12:34:56`
    #[default]
    Local,
    /// Utc date and time in an ISO 8601 like format.
    ///
    /// Examples: `2013-02-27 12:34:56Z`
    Utc,
    /// Elapsed time since the start of the program formatted in seconds with millisecond precision.
    ///
    /// Examples: `30020.289`
    Elapsed,
    /// No timestamp
    None,
}
