use chrono::NaiveDateTime;
use crate::infrastructure::integration::telegram::dto::Message;
use crate::infrastructure::service::command::enums::Type;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug)]
pub struct Command {
    pub str: String,
    pub r#type: Type,
    pub message: Message,
}
impl Command {
    pub fn new(str: String, r#type: Type, message: Message,) -> Self {
        Self { str, r#type, message }
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
        Self { code, stdout, stderr, message }
    }
}

pub struct Event {
    pub text: String,
    pub date: NaiveDateTime
}
impl Event {
    pub fn new(text: String, date: NaiveDateTime) -> Self {
        Self { text, date }
    }
}
impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.date.format(DATE_FORMAT).to_string().as_str(), self.text)
    }
}