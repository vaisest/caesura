use clap::Subcommand;

use crate::options::verify_options::VerifyOptions;
use crate::options::{RunnerOptions, SharedOptions, SpectrogramOptions, TargetOptions};

/// Cli sub-commands and arguments
#[derive(Subcommand, Debug, Clone)]
pub enum CommandArguments {
    /// Generate spectrograms for each track of a FLAC source.
    Spectrogram {
        #[command(flatten)]
        shared: SharedOptions,
        #[command(flatten)]
        runner: RunnerOptions,
        #[command(flatten)]
        spectrogram: SpectrogramOptions,
    },

    /// Transcode each track of a FLAC source to the target formats.
    Transcode {
        #[command(flatten)]
        shared: SharedOptions,
        #[command(flatten)]
        runner: RunnerOptions,
        #[command(flatten)]
        target: TargetOptions,
    },

    /// Verify a FLAC source is suitable for transcoding.
    Verify {
        #[command(flatten)]
        shared: SharedOptions,
        #[command(flatten)]
        target: TargetOptions,
        #[command(flatten)]
        verify: VerifyOptions,
    },
}
