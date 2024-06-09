use rogue_oxide::errors::AppError;
use rogue_oxide::formats::TargetFormatProvider;
use rogue_oxide::fs::DirectoryReader;
use rogue_oxide::hosting::HostBuilder;
use rogue_oxide::logging::{Debug, Logger};
use rogue_oxide::options::{Options, SharedOptions, TargetOptions};
use rogue_oxide::source::SourceProvider;
use rogue_oxide::testing::*;
use rogue_oxide::transcode::TranscodeCommand;

#[tokio::test]
async fn transcode_command() -> Result<(), AppError> {
    // Arrange
    Logger::init_new(Debug);
    let shared_options = TestOptionsFactory::from_with_env(SharedOptions {
        verbosity: Some(Debug),
        output: Some(TempDirectory::create("rogue_oxide")),
        ..SharedOptions::default()
    });
    let target_options = TestOptionsFactory::from(TargetOptions {
        allow_existing: Some(true),
        ..TargetOptions::default()
    });
    let output_dir = shared_options.get_value(|x| x.output.clone());
    let host = HostBuilder::new()
        .with_options(shared_options.clone())
        .with_options(target_options)
        .build();
    let provider = host.services.get_required_mut::<SourceProvider>();
    let transcoder = host.services.get_required::<TranscodeCommand>();
    let source = provider
        .write()
        .expect("Source provider should be writeable")
        .get_by_string(&shared_options.source.unwrap_or_default())
        .await
        .expect("Source provider should not fail");

    // Act
    transcoder.execute().await?;

    // Assert
    let generated_files = DirectoryReader::new()
        .with_extensions(vec!["flac", "mp3"])
        .read(&output_dir)
        .expect("Should be able to read dir");
    let targets = host.services.get_required::<TargetFormatProvider>();
    let target_count = targets.get(source.format, &source.existing).len();
    let expected_file_count = DirectoryReader::new()
        .with_extension("flac")
        .read(&source.directory)
        .expect("Should be able to read source dir")
        .len()
        * target_count;
    assert_eq!(generated_files.len(), expected_file_count);
    Ok(())
}
