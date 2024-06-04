use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Output;

use colored::Colorize;
use log::{trace, warn};
use tokio::fs::{copy, create_dir_all, hard_link, File};
use tokio::process::Command;

use crate::dependencies::CONVERT;
use crate::errors::{AppError, OutputHandler};

const IMAGE_EXTENSIONS: [&str; 4] = ["gif", "jpg", "jpeg", "png"];
const MAX_FILE_SIZE: u64 = 750_000;
const MAX_PIXEL_SIZE: u32 = 1920_u32;
const QUALITY: u32 = 90_u32;

pub struct AdditionalJob {
    pub id: String,
    pub source_path: PathBuf,
    pub output_dir: String,
    pub output_path: String,
    pub hard_link: bool,
    pub compress_images: bool,
    pub png_to_jpg: bool,
    pub extension: String,
}

impl AdditionalJob {
    #[allow(clippy::integer_division)]
    pub async fn execute(self) -> Result<(), AppError> {
        let file = File::open(&self.source_path)
            .await
            .or_else(|e| AppError::io(e, "open additional file"))?;
        let metadata = file
            .metadata()
            .await
            .or_else(|e| AppError::io(e, "read metadata of additional file"))?;
        let size = metadata.size();
        let is_large = size > MAX_FILE_SIZE;
        let is_image = IMAGE_EXTENSIONS.contains(&self.extension.as_str());
        if is_large && (!is_image || !self.compress_images) {
            warn!(
                "Including large {} ({} KB): {:?}",
                self.extension,
                size / 1_000,
                self.source_path
            );
        }
        create_dir_all(&self.output_dir)
            .await
            .or_else(|e| AppError::io(e, "create directories for additional file"))?;

        let verb = if is_large && is_image && self.compress_images {
            compress_image(&self.source_path, &self.output_path, self.png_to_jpg).await?;
            "Compressed"
        } else if self.hard_link {
            hard_link(&self.source_path, &self.output_path)
                .await
                .or_else(|e| AppError::io(e, "hard link additional file"))?;
            "Hard Linked"
        } else {
            copy(&self.source_path, &self.output_path)
                .await
                .or_else(|e| AppError::io(e, "copy additional file"))?;
            "Copied"
        };
        trace!(
            "{} {:?} to {}",
            verb.bold(),
            &self.source_path,
            &self.output_path
        );

        Ok(())
    }
}

async fn compress_image(
    source_path: &Path,
    output_path: &str,
    png_to_jpg: bool,
) -> Result<Output, AppError> {
    let mut output_path = output_path.to_owned();
    let extension = source_path
        .extension()
        .unwrap_or_default()
        .to_string_lossy();
    let extension = extension.as_ref();
    if png_to_jpg && extension == "png" {
        output_path = output_path
            .strip_suffix(extension)
            .expect("path should have extension")
            .to_owned()
            + "jpg";
    }
    let source_path = source_path.to_string_lossy().into_owned();
    trace!(
        "{} image to maximum {} px and {}% quality: {}",
        "Compressing".bold(),
        MAX_PIXEL_SIZE,
        QUALITY,
        source_path
    );
    let output = Command::new(CONVERT)
        .arg(source_path)
        .arg("-resize")
        .arg(format!("{MAX_PIXEL_SIZE}x{MAX_PIXEL_SIZE}>"))
        .arg("-quality")
        .arg(format!("{QUALITY}%"))
        .arg(output_path)
        .output()
        .await
        .or_else(|e| AppError::io(e, "execute compress image"))?;
    OutputHandler::execute(output, "compress image", "convert")
}
