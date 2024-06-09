use crate::batch::BatchCommand;
use di::ServiceProvider;

use crate::cli::ArgumentsParser;
use crate::cli::CommandArguments::*;
use crate::errors::AppError;
use crate::logging::*;
use crate::spectrogram::SpectrogramCommand;
use crate::transcode::TranscodeCommand;
use crate::upload::UploadCommand;
use crate::verify::VerifyCommand;

/// Application host, responsible for executing the application
///
/// [`HostBuilder`] takes care of building the [Host] and loading the
/// dependency injection [`ServiceProvider`].
pub struct Host {
    /// Dependency injection service provider
    pub services: ServiceProvider,
}

impl Host {
    #[must_use]
    pub fn new(services: ServiceProvider) -> Self {
        Host { services }
    }

    /// Execute the application
    ///
    /// 1. Configure logging
    /// 2. Determine the command to execute
    /// 3. Execute the command
    pub async fn execute(&self) -> Result<bool, AppError> {
        let logger = self.services.get_required::<Logger>();
        Logger::init(logger);
        match ArgumentsParser::get_or_exit() {
            Batch { .. } => {
                self.services
                    .get_required_mut::<BatchCommand>()
                    .write()
                    .expect("BatchCommand should be available to write")
                    .execute()
                    .await
            }
            Spectrogram { .. } => {
                self.services
                    .get_required::<SpectrogramCommand>()
                    .execute()
                    .await
            }
            Transcode { .. } => {
                self.services
                    .get_required::<TranscodeCommand>()
                    .execute()
                    .await
            }
            Upload { .. } => {
                self.services
                    .get_required_mut::<UploadCommand>()
                    .write()
                    .expect("UploadCommand should be available to write")
                    .execute()
                    .await
            }
            Verify { .. } => {
                self.services
                    .get_required_mut::<VerifyCommand>()
                    .write()
                    .expect("VerifyCommand should be available to write")
                    .execute()
                    .await
            }
        }
    }
}
