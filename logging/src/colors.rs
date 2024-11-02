use colored::{ColoredString, Colorize, CustomColor};

const GRAY: u8 = 168;
const DARK_GRAY: u8 = 112;

pub trait Colors {
    type Error;

    fn gray(&self) -> ColoredString;

    fn dark_gray(&self) -> ColoredString;
}

impl Colors for &str {
    type Error = ();

    fn gray(&self) -> ColoredString {
        self.custom_color(CustomColor::new(GRAY, GRAY, GRAY))
    }

    fn dark_gray(&self) -> ColoredString {
        self.custom_color(CustomColor::new(DARK_GRAY, DARK_GRAY, DARK_GRAY))
    }
}

impl Colors for String {
    type Error = ();

    fn gray(&self) -> ColoredString {
        self.custom_color(CustomColor::new(GRAY, GRAY, GRAY))
    }

    fn dark_gray(&self) -> ColoredString {
        self.custom_color(CustomColor::new(DARK_GRAY, DARK_GRAY, DARK_GRAY))
    }
}

impl Colors for ColoredString {
    type Error = ();

    fn gray(&self) -> ColoredString {
        self.clone()
            .custom_color(CustomColor::new(GRAY, GRAY, GRAY))
    }

    fn dark_gray(&self) -> ColoredString {
        self.clone()
            .custom_color(CustomColor::new(DARK_GRAY, DARK_GRAY, DARK_GRAY))
    }
}
