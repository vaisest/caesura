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
    /// Should hard links be used when copying files?
    ///
    /// Default: `false`
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub hard_link: Option<bool>,

    /// Should compression of images be disabled?
    ///
    /// Default: `false`
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub no_image_compression: Option<bool>,

    /// Maximum file size in bytes beyond which images are compressed.
    ///
    /// Default: `750000`
    ///
    /// Only applies to image files.
    #[arg(long)]
    pub max_file_size: Option<u64>,

    /// Maximum size in pixels for images
    ///
    /// Default: `1280`
    ///
    /// Only applied if the image is greater than `max_file_size`.
    #[arg(long)]
    pub max_pixel_size: Option<u32>,

    /// Quality percentage to apply for jpg compression.
    ///
    /// Default: `80`
    ///
    /// Only applied if the image is greated than `max_file_size`.
    #[arg(long)]
    pub jpg_quality: Option<u8>,

    /// Should conversion of png images to jpg be disabled?
    ///
    /// Default: `false`
    ///
    /// Only applied if the image is greater than `max_file_size`.
    #[arg(long, default_value = None, action = ArgAction::SetTrue)]
    pub no_png_to_jpg: Option<bool>,
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

    fn merge(&mut self, alternative: &Self) {
        if self.hard_link.is_none() {
            self.hard_link = alternative.hard_link;
        }
        if self.no_image_compression.is_none() {
            self.no_image_compression = alternative.no_image_compression;
        }
        if self.no_png_to_jpg.is_none() {
            self.no_png_to_jpg = alternative.no_png_to_jpg;
        }
        if self.max_file_size.is_none() {
            self.max_file_size = alternative.max_file_size;
        }
        if self.max_pixel_size.is_none() {
            self.max_pixel_size = alternative.max_pixel_size;
        }
        if self.jpg_quality.is_none() {
            self.jpg_quality = alternative.jpg_quality;
        }
    }

    fn apply_defaults(&mut self) {
        if self.hard_link.is_none() {
            self.hard_link = Some(false);
        }
        if self.no_image_compression.is_none() {
            self.no_image_compression = Some(false);
        }
        if self.no_png_to_jpg.is_none() {
            self.no_png_to_jpg = Some(false);
        }
        if self.max_file_size.is_none() {
            self.max_file_size = Some(750_000);
        }
        if self.max_pixel_size.is_none() {
            self.max_pixel_size = Some(1280);
        }
        if self.jpg_quality.is_none() {
            self.jpg_quality = Some(80);
        }
    }

    #[must_use]
    fn validate(&self) -> bool {
        true
    }

    #[must_use]
    fn from_args() -> Option<Self> {
        let options = match ArgumentsParser::get() {
            Some(Batch { file, .. }) => file,
            Some(Transcode { file, .. }) => file,
            _ => return None,
        };
        let mut options = options;
        if options.hard_link == Some(false) {
            options.hard_link = None;
        }
        if options.no_image_compression == Some(false) {
            options.no_image_compression = None;
        }
        if options.no_png_to_jpg == Some(false) {
            options.no_png_to_jpg = None;
        }
        Some(options)
    }

    fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }

    fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

impl Display for FileOptions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(yaml) = serde_yaml::to_string(self) {
            yaml
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
