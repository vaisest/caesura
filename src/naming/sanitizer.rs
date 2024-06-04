const RESTRICTED: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

const REPLACEMENT: &str = "-";

pub struct Sanitizer;

impl Sanitizer {
    #[must_use]
    pub fn execute(input: String) -> String {
        input.replace(&RESTRICTED[..], REPLACEMENT)
    }
}
