use crate::built_info::PKG_NAME;
use crate::errors::AppError;
use crate::hosting::HostBuilder;
use crate::options::TargetOptions;
use crate::testing::options::TestOptionsFactory;
use crate::verify::VerifyCommand;
use rogue_logging::Logger;

#[tokio::test]
async fn verify_command() -> Result<(), AppError> {
    // Arrange
    Logger::force_init(PKG_NAME.to_owned());
    let target_options = TestOptionsFactory::from(TargetOptions {
        allow_existing: Some(true),
        ..TargetOptions::default()
    });
    let host = HostBuilder::new().with_options(target_options).build();
    let verifier = host.services.get_required_mut::<VerifyCommand>();
    let mut verifier = verifier
        .write()
        .expect("verifier should be available to write");

    // Act
    let _is_verified = verifier.execute_cli().await?;

    // Assert not required
    Ok(())
}
