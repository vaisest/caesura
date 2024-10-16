use crate::errors::AppError;
use crate::formats::TargetFormatProvider;
use crate::fs::DirectoryReader;
use crate::hosting::HostBuilder;
use crate::logging::Logger;
use crate::options::{SharedOptions, TargetOptions};
use crate::source::SourceProvider;
use crate::testing::options::TestOptionsFactory;
use crate::testing::*;
use crate::transcode::TranscodeCommand;

#[tokio::test]
async fn transcode_command() -> Result<(), AppError> {
    // Arrange
    Logger::force_init();
    let shared_options = TestOptionsFactory::from(SharedOptions {
        output: Some(TempDirectory::create("caesura")),
        ..SharedOptions::default()
    });
    let target_options = TestOptionsFactory::from(TargetOptions {
        allow_existing: Some(true),
        ..TargetOptions::default()
    });
    let output_dir = shared_options.output.clone().expect("output should be set");
    let host = HostBuilder::new()
        .with_options(shared_options.clone())
        .with_options(target_options)
        .build();
    let provider = host.services.get_required_mut::<SourceProvider>();
    let transcoder = host.services.get_required::<TranscodeCommand>();
    let source = provider
        .write()
        .expect("Source provider should be writeable")
        .get_from_options()
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
