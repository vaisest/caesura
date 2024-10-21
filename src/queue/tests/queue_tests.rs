use super::super::Queue;
use crate::queue::TimeStamp;
use crate::testing::TempDirectory;
use crate::upload::UploadStatus;
use std::fs::File;
use std::io::{Read, Write};

const QUEUE_YAML: &str = r"abc_transcoded:
  name: 1 Transcoded but not uploaded
  path: /path/to/abc_transcoded.torrent
  hash: abc_transcoded
  indexer: abc
  transcode:
    success: true
    completed: 2021-08-01T00:00:00Z
abc_uploaded:
  name: 2 Transcoded and uploaded
  path: /path/to/abc_uploaded.torrent
  hash: abc_uploaded
  indexer: abc
  transcode:
    success: true
    completed: 2021-08-01T00:00:00Z
  upload:
    success: true
    completed: 2021-08-01T00:00:00Z
cba_item:
  name: 3 This item should be skipped
  path: /path/to/cba_item.torrent
  hash: cba_item
  indexer: cba
skipped_item:
  name: 4 This item should be skipped
  path: /path/to/skipped_item.torrent
  hash: skipped_item
  indexer: abc
  skip: Skipped for a reason
";

#[test]
fn queue_end_to_end() {
    // Arrange
    let file_name = "queue.yml";
    let dir = TempDirectory::create("queue");
    let path = dir.join(file_name);
    let mut file = File::create(path.clone()).unwrap();
    file.write_all(QUEUE_YAML.as_bytes()).unwrap();
    let mut queue = Queue::from_path(path.clone());

    // Act LOAD
    queue.load().unwrap();

    // Assert
    assert_eq!(queue.len(), 4);

    // Act SAVE
    queue.save().unwrap();

    // Assert
    let mut file = File::open(path.clone()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    assert_eq!(content, QUEUE_YAML);

    // Act GET_UNPROCESSED
    let indexer = "abc".to_owned();
    let with_transcoded = queue.get_unprocessed(indexer.clone(), true);
    let without_transcoded = queue.get_unprocessed(indexer, false);

    // Assert
    assert_eq!(with_transcoded.len(), 1);
    assert_eq!(without_transcoded.len(), 0);

    // Act GET
    let hash = "abc_transcoded";
    let item_before = queue.get(hash).unwrap();

    // Assert
    assert!(item_before.upload.is_none());

    // Act SET
    let status = UploadStatus {
        success: true,
        formats: None,
        completed: TimeStamp::now(),
        errors: None,
    };
    queue.set_upload(hash.to_owned(), status);

    // Assert
    let item_after = queue.get(hash).unwrap();
    assert!(item_after.upload.is_some());
}
