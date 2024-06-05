use std::fmt::{Display, Formatter};

use clap::{ArgAction, Args};
use di::{injectable, Ref};
use serde::{Deserialize, Serialize};

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use crate::options::{Options, OptionsProvider};

/// Options for including additional files during [`TranscodeCommand`]
#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct FileOptions {
    /// Use hard links when copying files
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub hard_link: Option<bool>,

    /// Should images greater than 750 KB be compressed?
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub compress_images: Option<bool>,

    /// Should png images be converted to jpg?
    ///
    /// Only applied if the image is greated than 750 KB and `compress_images` is true.
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub png_to_jpg: Option<bool>,
}

#[injectable]
impl FileOptions {
    fn new(provider: Ref<OptionsProvider>) -> Self {
        provider.get()
    }
}

impl Options for FileOptions {
    fn get_name() -> String {
        "Additional File Options".to_owned()
    }

    /// Merge the current options with an alternative set of options
    ///
    /// The current options will take precedence over the alternative options
    fn merge(&mut self, alternative: &Self) {
        if self.hard_link.is_none() {
            self.hard_link = alternative.hard_link;
        }
        if self.compress_images.is_none() {
            self.compress_images = alternative.compress_images;
        }
        if self.png_to_jpg.is_none() {
            self.png_to_jpg = alternative.png_to_jpg;
        }
    }

    fn apply_defaults(&mut self) {
        if self.hard_link.is_none() {
            self.hard_link = Some(false);
        }
        if self.compress_images.is_none() {
            self.compress_images = Some(false);
        }
        if self.png_to_jpg.is_none() {
            self.png_to_jpg = Some(false);
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
            Some(Transcode { file, .. }) => file,
            _ => return None,
        };
        let mut options = options;
        if options.hard_link == Some(false) {
            options.hard_link = None;
        }
        if options.compress_images == Some(false) {
            options.compress_images = None;
        }
        if options.png_to_jpg == Some(false) {
            options.png_to_jpg = None;
        }
        Some(options)
    }

    fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }
}

impl Display for FileOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(json) = serde_json::to_string_pretty(self) {
            json
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
