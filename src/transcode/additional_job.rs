use colored::Colorize;
use log::trace;

use crate::dependencies::CONVERT;
use crate::errors::{AppError, OutputHandler};
use crate::transcode::resize::Resize;

pub struct AdditionalJob {
    pub id: String,
    pub resize: Resize,
}

impl AdditionalJob {
    #[allow(clippy::integer_division)]
    pub async fn execute(self) -> Result<(), AppError> {
        trace!(
            "{} image to maximum {} px and {}% quality: {}",
            "Resizing".bold(),
            self.resize.max_pixel_size,
            self.resize.quality,
            self.resize.input.display()
        );
        let info = self.resize.to_info();
        trace!("{info}");
        let output = info
            .to_command()
            .output()
            .await
            .or_else(|e| AppError::command(e, "execute resize image", CONVERT))?;
        OutputHandler::execute(output, "resize image", "convert")?;
        Ok(())
    }
}
