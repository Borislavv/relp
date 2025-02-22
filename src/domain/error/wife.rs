use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct WifeMessagesVecIsEmptyError {}

impl WifeMessagesVecIsEmptyError {
    pub fn new() -> WifeMessagesVecIsEmptyError {
        WifeMessagesVecIsEmptyError {}
    }
}

impl fmt::Display for WifeMessagesVecIsEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Messages Vec is empty, unrecoverable error.")
    }
}

impl Error for WifeMessagesVecIsEmptyError {}
