use crate::errors::AppError;
use crate::hosting::HostBuilder;
use crate::logging::{Debug, Logger};
use crate::options::{SharedOptions, TargetOptions};
use crate::testing::options::TestOptionsFactory;
use crate::verify::VerifyCommand;

#[tokio::test]
async fn verify_command() -> Result<(), AppError> {
    // Arrange
    Logger::init_new(Debug);
    let shared_options = TestOptionsFactory::from(SharedOptions {
        verbosity: Some(Debug),
        ..SharedOptions::default()
    });
    let target_options = TestOptionsFactory::from(TargetOptions {
        allow_existing: Some(true),
        ..TargetOptions::default()
    });
    let host = HostBuilder::new()
        .with_options(shared_options.clone())
        .with_options(target_options)
        .build();
    let verifier = host.services.get_required_mut::<VerifyCommand>();
    let mut verifier = verifier
        .write()
        .expect("verifier should be available to write");

    // Act
    let _is_verified = verifier.execute().await?;

    // Assert not required
    Ok(())
}
