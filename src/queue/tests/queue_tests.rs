use super::super::Queue;
use crate::db::Hash;
use crate::errors::AppError;
use crate::queue::{QueueItem, TimeStamp};
use crate::source::SourceIssue;
use crate::testing::TempDirectory;
use crate::transcode::TranscodeStatus;
use crate::upload::UploadStatus;
use crate::verify::VerifyStatus;
use std::path::PathBuf;

#[tokio::test]
async fn queue_get_unprocessed() -> Result<(), AppError> {
    // Arrange
    let new = Hash::<20>::from_string("1100000000000000000000000000000000000000")?;
    let verified = Hash::<20>::from_string("2200000000000000000000000000000000000000")?;
    let not_verified = Hash::<20>::from_string("3300000000000000000000000000000000000000")?;
    let transcoded = Hash::<20>::from_string("4400000000000000000000000000000000000000")?;
    let not_transcoded = Hash::<20>::from_string("5500000000000000000000000000000000000000")?;
    let uploaded = Hash::<20>::from_string("6600000000000000000000000000000000000000")?;
    let not_uploaded = Hash::<20>::from_string("7700000000000000000000000000000000000000")?;

    let mut queue = Queue::from_path(TempDirectory::create("caesura"));
    queue
        .set(QueueItem {
            name: "NEW".to_owned(),
            path: PathBuf::new(),
            hash: new,
            ..QueueItem::default()
        })
        .await?;
    queue
        .set(QueueItem {
            name: "VERIFIED".to_owned(),
            path: PathBuf::new(),
            hash: verified,
            verify: Some(VerifyStatus::verified()),
            ..QueueItem::default()
        })
        .await?;
    queue
        .set(QueueItem {
            name: "VERIFY FAILURE".to_owned(),
            path: PathBuf::new(),
            hash: not_verified,
            verify: Some(VerifyStatus::from_issue(SourceIssue::IdError {
                details: "missing id".to_owned(),
            })),
            ..QueueItem::default()
        })
        .await?;
    queue
        .set(QueueItem {
            name: "TRANSCODED".to_owned(),
            path: PathBuf::new(),
            hash: transcoded,
            verify: Some(VerifyStatus::verified()),
            transcode: Some(TranscodeStatus {
                success: true,
                completed: TimeStamp::now(),
                formats: None,
                additional: None,
                error: None,
            }),
            ..QueueItem::default()
        })
        .await?;
    queue
        .set(QueueItem {
            name: "TRANSCODE FAILURE".to_owned(),
            path: PathBuf::new(),
            hash: not_transcoded,
            verify: Some(VerifyStatus::verified()),
            transcode: Some(TranscodeStatus {
                success: false,
                completed: TimeStamp::now(),
                formats: None,
                additional: None,
                error: None,
            }),
            ..QueueItem::default()
        })
        .await?;
    queue
        .set(QueueItem {
            name: "UPLOADED".to_owned(),
            path: PathBuf::new(),
            hash: uploaded,
            verify: Some(VerifyStatus::verified()),
            transcode: Some(TranscodeStatus {
                success: true,
                completed: TimeStamp::now(),
                formats: None,
                additional: None,
                error: None,
            }),
            upload: Some(UploadStatus {
                success: true,
                completed: TimeStamp::now(),
                formats: None,
                errors: None,
            }),
            ..QueueItem::default()
        })
        .await?;
    queue
        .set(QueueItem {
            name: "UPLOAD FAILURE".to_owned(),
            path: PathBuf::new(),
            hash: not_uploaded,
            verify: Some(VerifyStatus::verified()),
            transcode: Some(TranscodeStatus {
                success: true,
                completed: TimeStamp::now(),
                formats: None,
                additional: None,
                error: None,
            }),
            upload: Some(UploadStatus {
                success: false,
                completed: TimeStamp::now(),
                formats: None,
                errors: None,
            }),
            ..QueueItem::default()
        })
        .await?;

    // Assert
    let verify = queue.get_unprocessed(String::new(), false, false).await?;
    assert_eq!(verify, vec![new]);
    let transcode = queue.get_unprocessed(String::new(), true, false).await?;
    assert_eq!(transcode, vec![new, verified]);
    let upload = queue.get_unprocessed(String::new(), true, true).await?;
    assert_eq!(upload, vec![new, transcoded, verified]);

    Ok(())
}
