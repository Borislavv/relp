use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NoEntryWasFoundError {
}
impl NoEntryWasFoundError {
    pub fn new() -> NoEntryWasFoundError {
        NoEntryWasFoundError {}
    }
}
impl fmt::Display for NoEntryWasFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No entry was found by given key.")
    }
}
impl Error for NoEntryWasFoundError {
}

#[derive(Debug)]
pub struct SendOnClosedChannelError {
    previous: String,
}
impl SendOnClosedChannelError {
    pub fn new(previous: String) -> SendOnClosedChannelError {
        SendOnClosedChannelError { previous }
    }
}
impl fmt::Display for SendOnClosedChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Send data on closed channel (the receiver side has gone away). Previous: {}", self.previous)
    }
}
impl Error for SendOnClosedChannelError {
}

#[derive(Debug)]
pub struct AlreadyExistsError {
    previous: String,
}
impl AlreadyExistsError {
    pub fn new(previous: String) -> AlreadyExistsError {
        AlreadyExistsError { previous }
    }
}
impl fmt::Display for AlreadyExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The passed key already present in the map. Previous: {}", self.previous)
    }
}
impl Error for AlreadyExistsError {
}