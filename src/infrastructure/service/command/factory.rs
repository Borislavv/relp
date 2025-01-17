use std::sync::{Arc, Mutex};
use service::command::enumerate::Type;
use service::command::model::Command;
use crate::infrastructure::service;
use crate::infrastructure::integration;
use integration::telegram::model::Message;
use crate::infrastructure::service::command::model::Event;

const CMD_PREFIX: &str = "/cmd";
const NOTE_PREFIX: &str = "/note";
const EVENT_PREFIX: &str = "/event";
const PING_PREFIX: &str = "/ping";
const NOT_FOUND_PREFIX: &str = "/mirror";

pub trait Factoryer: Send + Sync {
    fn make(&self, msg: Message) -> Command;
}

pub struct CommandFactory {
    note_mutex: Arc<Mutex<Vec<String>>>,
    event_mutex: Arc<Mutex<Vec<Event>>>,
}

impl CommandFactory {
    pub fn new() -> CommandFactory {
        CommandFactory {
            note_mutex: Arc::new(Mutex::new(vec![])),
            event_mutex: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl Factoryer for CommandFactory {
    fn make(&self, msg: Message) -> Command {
        let (cmd_type, prefix) = match msg.text.clone() {
            str if str.starts_with(CMD_PREFIX) => (Type::Cmd, CMD_PREFIX),
            str if str.starts_with(NOTE_PREFIX) => (Type::Note, NOTE_PREFIX),
            str if str.starts_with(EVENT_PREFIX) => (Type::Event, EVENT_PREFIX),
            str if str.starts_with(PING_PREFIX) => (Type::Ping, PING_PREFIX),
            _ => (Type::NotFound, NOT_FOUND_PREFIX)
        };

        Command::new(
            msg.text.clone().replace(prefix, ""),
            cmd_type,
            msg,
            self.note_mutex.clone(),
            self.event_mutex.clone(),
        )
    }
}