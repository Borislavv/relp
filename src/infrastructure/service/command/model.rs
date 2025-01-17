use std::sync::{Arc, Mutex};
use chrono::NaiveDateTime;
use crate::infrastructure::integration::telegram::model::Message;
use crate::infrastructure::service::command::{command, enumerate};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Clone)]
pub struct Command {
    pub str: String,
    pub r#type: enumerate::Type,
    pub message: Message,
    note_mutex: Arc<Mutex<Vec<String>>>,
    event_mutex: Arc<Mutex<Vec<Event>>>,
}
impl Command {
    pub fn new(
        str: String,
        r#type: enumerate::Type,
        message: Message,
        note_mutex: Arc<Mutex<Vec<String>>>,
        event_mutex: Arc<Mutex<Vec<Event>>>,
    ) -> Self {
        Self { str, r#type, message, note_mutex, event_mutex }
    }
}
impl command::Executable for Command {
    fn exec(&self) -> Exit {
        let cmd = self.clone();

        let executable: Box<dyn command::Executable> = match self.r#type {
            enumerate::Type::Ping => Box::new(command::Ping::new()),
            enumerate::Type::Note => Box::new(command::Note::new(cmd, self.note_mutex.clone())),
            enumerate::Type::Event => Box::new(command::Event::new(cmd, self.event_mutex.clone())),
            enumerate::Type::Cmd => Box::new(command::Cmd::new(cmd)),
            _ => Box::new(command::NotFound::new(cmd)),
        };

        executable.exec()
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