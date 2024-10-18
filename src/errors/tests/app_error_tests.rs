use crate::errors::AppError;

#[test]
fn test_explained_serialization() {
    // Arrange
    let error =
        AppError::explained::<()>("PerformAction", "Something went wrong".to_owned()).unwrap_err();

    // Act
    let yaml_output = serde_yaml::to_string(&error).unwrap();

    // Assert
    let expected_output = "action: PerformAction
message: Something went wrong
";
    assert_eq!(yaml_output, expected_output);
}

#[test]
fn test_external_serialization() {
    // Arrange
    let error =
        AppError::external::<()>("PerformAction", "TestDomain", "External failure".to_owned())
            .unwrap_err();

    // Act
    let yaml_output = serde_yaml::to_string(&error).unwrap();

    // Assert
    let expected_output = "action: PerformAction
domain: TestDomain
message: External failure
";
    assert_eq!(yaml_output, expected_output);
}

#[test]
fn test_unexpected_serialization() {
    // Arrange
    let error = AppError::unexpected::<()>(
        "PerformAction",
        "An unexpected error occurred",
        "ExpectedValue".to_owned(),
        "ActualValue".to_owned(),
    )
    .unwrap_err();

    // Act
    let yaml_output = serde_yaml::to_string(&error).unwrap();

    // Assert
    let expected_output = "action: PerformAction
message: An unexpected error occurred
actual: ActualValue
expected: ExpectedValue
";
    assert_eq!(yaml_output, expected_output);
}
