use std::fmt;
use std::error::Error;

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
        write!(f, "Send data on closed channel (the receiver side has gone away).")
    }
}
impl Error for NoEntryWasFoundError {
}