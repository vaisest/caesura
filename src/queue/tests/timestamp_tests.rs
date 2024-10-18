use crate::queue::TimeStamp;
use chrono::{DateTime, Utc};

#[test]
fn test_serialize_timestamp() {
    let datetime = DateTime::parse_from_rfc3339("2024-10-18T12:34:56Z")
        .expect("")
        .with_timezone(&Utc);
    let timestamp = TimeStamp { datetime };

    let serialized = serde_json::to_string(&timestamp).expect("");
    assert_eq!(serialized, "\"2024-10-18T12:34:56+00:00\"");
}

#[test]
fn deserialize_timestamp() {
    // Arrange
    let json = "\"2024-10-18T12:34:56+00:00\"";
    let deserialized: TimeStamp = serde_json::from_str(json).expect("");

    // Act
    let expected = DateTime::parse_from_rfc3339("2024-10-18T12:34:56Z")
        .expect("")
        .with_timezone(&Utc);
    let expected = TimeStamp { datetime: expected };

    // Assert
    assert_eq!(deserialized, expected);
}

#[test]
fn round_trip_serialization() {
    // Arrange
    let datetime = Utc::now();
    let timestamp = TimeStamp { datetime };

    // Act
    let serialized = serde_json::to_string(&timestamp).expect("");
    let deserialized: TimeStamp = serde_json::from_str(&serialized).expect("");

    // Assert
    assert_eq!(timestamp, deserialized);
}

#[test]
fn invalid_timestamp_format() {
    // Arrange
    let invalid_json = "\"invalid-date-format\"";

    // Act
    let result: Result<TimeStamp, _> = serde_json::from_str(invalid_json);

    // Assert
    assert!(
        result.is_err(),
        "Deserialization should fail with invalid date format"
    );
}
