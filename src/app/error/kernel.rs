use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NotBootedKernelError {}

impl NotBootedKernelError {
    pub fn new() -> NotBootedKernelError {
        NotBootedKernelError {}
    }
}

impl fmt::Display for NotBootedKernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Trying to access non-booted kernel.")
    }
}

impl Error for NotBootedKernelError {}
