use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct DateTimeParseError {
}

impl DateTimeParseError {
    pub fn new() -> DateTimeParseError {
        DateTimeParseError {}
    }
}

impl fmt::Display for DateTimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DateTime parse error.")
    }
}

impl Error for DateTimeParseError {
}