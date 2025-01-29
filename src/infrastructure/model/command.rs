use crate::domain::r#enum::command::Type;
use crate::infrastructure::integration::telegram::model::Message;

pub const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Clone)]
pub struct Command {
    pub str: String,
    pub r#type: Type,
    pub message: Message,
}
impl Command {
    pub fn new(str: String, r#type: Type, message: Message) -> Self {
        Self {
            str,
            r#type,
            message,
        }
    }
}

#[derive(Debug)]
pub struct Exit {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
    pub message: Option<Message>,
}
impl Exit {
    pub fn new(code: i32, stdout: String, stderr: String, message: Option<Message>) -> Self {
        Self {
            code,
            stdout,
            stderr,
            message,
        }
    }
}
