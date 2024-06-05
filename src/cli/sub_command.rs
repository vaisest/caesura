use clap::Subcommand;

use crate::options::{SharedOptions, SpectrogramOptions, TranscodeOptions};

/// Cli sub-commands and arguments
#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    /// Generate spectrograms for each track of a FLAC source.
    Spectrogram {
        #[command(flatten)]
        shared: SharedOptions,
        #[command(flatten)]
        spectrogram: SpectrogramOptions,
    },

    /// Transcode each track of a FLAC source to the target formats.
    Transcode {
        #[command(flatten)]
        shared: SharedOptions,
        #[command(flatten)]
        transcode: TranscodeOptions,
    },

    /// Verify a FLAC source is suitable for transcoding.
    Verify {
        #[command(flatten)]
        shared: SharedOptions,
        #[command(flatten)]
        transcode: TranscodeOptions,
    },
}
