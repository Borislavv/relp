use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UnknownMessageTypeError {}

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

impl Error for OffsetFetchError {}

#[derive(Debug)]
pub struct OffsetFetchError {
    parent: String
}

impl OffsetFetchError {
    pub fn new(parent: Option<Box<dyn Error>>) -> OffsetFetchError {
        OffsetFetchError {
            parent: match parent {
                Some(parent) => parent.to_string(),
                None => String::from("unknown reason"),
            }
        }
    }
}

impl fmt::Display for OffsetFetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to fetch offset: {}.", self.parent.to_string())
    }
}

impl Error for OffsetFetchError {}
