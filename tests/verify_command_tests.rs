use rogue_oxide::errors::AppError;
use rogue_oxide::hosting::HostBuilder;
use rogue_oxide::logging::{Debug, Logger};
use rogue_oxide::options::{SharedOptions, TargetOptions};
use rogue_oxide::testing::*;
use rogue_oxide::verify::VerifyCommand;

#[tokio::test]
async fn verify_command() -> Result<(), AppError> {
    // Arrange
    Logger::init_new(Debug);
    let shared_options = TestOptionsFactory::from_with_env(SharedOptions {
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
