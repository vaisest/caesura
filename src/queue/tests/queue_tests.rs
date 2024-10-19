use super::super::Queue;
use crate::testing::TempDirectory;
use std::fs::File;
use std::io::{Read, Write};

const QUEUE_YAML: &str = r"abc_transcoded:
  name: 1 Transcoded but not uploaded
  hash: abc_transcoded
  indexer: abc
  transcoded: true
abc_uploaded:
  name: 2 Transcoded and uploaded
  hash: abc_uploaded
  indexer: abc
  transcoded: true
  uploaded: true
cba_item:
  name: 3 This item should be skipped
  hash: cba_item
  indexer: cba
skipped_item:
  name: 4 This item should be skipped
  hash: skipped_item
  indexer: abc
  skipped: Skipped for a reason
";

#[test]
fn queue_end_to_end() {
    // Arrange
    let file_name = "queue.yml";
    let dir = TempDirectory::create("queue");
    let path = dir.join(file_name);
    let mut file = File::create(path.clone()).unwrap();
    file.write_all(QUEUE_YAML.as_bytes()).unwrap();
    let mut queue = Queue::new(path.clone());

    // Act LOAD
    queue.load(path.clone()).unwrap();

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
    let with_transcoded = queue.get_unprocessed(indexer.clone(), false);
    let without_transcoded = queue.get_unprocessed(indexer, true);

    // Assert
    assert_eq!(with_transcoded.len(), 1);
    assert_eq!(without_transcoded.len(), 0);

    // Act GET
    let hash = "abc_transcoded";
    let item_before = queue.get(hash).unwrap().clone();
    // Item has to be cloned so that we can then use the queue as mutable
    queue.set_uploaded(hash.to_owned());
    let item_after = queue.get(hash).unwrap();
    assert_eq!(item_before.uploaded, None);
    assert_eq!(item_after.uploaded, Some(true));
}
