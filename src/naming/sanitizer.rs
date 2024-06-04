const RESTRICTED: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

const REPLACEMENT: &str = "-";

pub struct Sanitizer;

impl Sanitizer {
    pub fn execute(input: String) -> String {
        input.replace(&RESTRICTED[..], REPLACEMENT)
    }
}
