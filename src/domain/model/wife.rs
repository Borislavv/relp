use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub text: String,
}

impl Message {
    pub fn new(text: String) -> Message {
        Message { text }
    }
}
