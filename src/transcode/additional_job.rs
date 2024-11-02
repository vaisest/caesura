use crate::dependencies::CONVERT;
use crate::errors::{command_error, OutputHandler};
use crate::transcode::resize::Resize;
use colored::Colorize;
use log::trace;
use rogue_logging::Error;

pub struct AdditionalJob {
    pub id: String,
    pub resize: Resize,
}

impl AdditionalJob {
    #[allow(clippy::integer_division)]
    pub async fn execute(self) -> Result<(), Error> {
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
            .map_err(|e| command_error(e, "execute resize image", CONVERT))?;
        OutputHandler::execute(output, "resize image", "convert")?;
        Ok(())
    }
}
