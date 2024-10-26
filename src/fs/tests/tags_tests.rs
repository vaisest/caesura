use super::super::*;

#[test]
fn valid_vinyl_formats() {
    assert_eq!(get_numeric_from_vinyl_format("A1"), Some((1, 1)));
    assert_eq!(get_numeric_from_vinyl_format("B6"), Some((2, 6)));
    assert_eq!(get_numeric_from_vinyl_format("C8"), Some((3, 8)));
    assert_eq!(get_numeric_from_vinyl_format("Z9"), Some((26, 9)));
}

#[test]
fn invalid_vinyl_formats() {
    assert_eq!(get_numeric_from_vinyl_format(""), None);
    assert_eq!(get_numeric_from_vinyl_format("A"), None);
    assert_eq!(get_numeric_from_vinyl_format("A12"), None);
    assert_eq!(get_numeric_from_vinyl_format("12"), None);
    assert_eq!(get_numeric_from_vinyl_format("1A"), None);
}
