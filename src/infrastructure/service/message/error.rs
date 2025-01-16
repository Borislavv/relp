use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UnknownMessageTypeError {
}

impl UnknownMessageTypeError {
    pub fn new() -> UnknownMessageTypeError {
        UnknownMessageTypeError {}
    }
}

impl fmt::Display for UnknownMessageTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown message type provided.")
    }
}

impl Error for UnknownMessageTypeError {
}