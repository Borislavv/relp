use crate::domain::model::command::{
    Event, EventCmd, ExecCmd, NotFoundCmd, Note, NoteCmd, PingCmd,
};
use crate::domain::model::event::ExecutableEvent;
use crate::domain::r#enum::command::Type;
use crate::domain::service::event::r#loop::EventLoop;
use crate::infrastructure::integration;
use crate::infrastructure::model::command::Command;
use integration::telegram::model::Message;
use std::sync::{Arc, Mutex};

const CMD_PREFIX: &str = "/cmd";
const NOTE_PREFIX: &str = "/note";
const EVENT_PREFIX: &str = "/event";
const PING_PREFIX: &str = "/ping";
const NOT_FOUND_PREFIX: &str = "/mirror";

pub trait Factoryer: Send + Sync {
    fn make(&self, msg: Message) -> Box<dyn ExecutableEvent>;
}

pub struct CommandFactory {
    note_mutex: Arc<Mutex<Vec<Note>>>,
    event_mutex: Arc<Mutex<Vec<Event>>>,
    event_loop: Arc<Box<dyn EventLoop>>,
}

impl CommandFactory {
    pub fn new(
        event_mutex: Arc<Mutex<Vec<Event>>>,
        event_loop: Arc<Box<dyn EventLoop>>,
    ) -> CommandFactory {
        CommandFactory {
            note_mutex: Arc::new(Mutex::new(vec![])),
            event_mutex,
            event_loop,
        }
    }
}

impl Factoryer for CommandFactory {
    fn make(&self, msg: Message) -> Box<dyn ExecutableEvent> {
        let (cmd_type, prefix) = match msg.text.clone() {
            str if str.starts_with(CMD_PREFIX) => (Type::Exec, CMD_PREFIX),
            str if str.starts_with(NOTE_PREFIX) => (Type::Note, NOTE_PREFIX),
            str if str.starts_with(EVENT_PREFIX) => (Type::Event, EVENT_PREFIX),
            str if str.starts_with(PING_PREFIX) => (Type::Ping, PING_PREFIX),
            _ => (Type::NotFound, NOT_FOUND_PREFIX),
        };

        let cmd = Command::new(msg.text.clone().replace(prefix, ""), cmd_type.clone(), msg);

        match cmd_type {
            Type::Ping => Box::new(PingCmd::new(cmd)),
            Type::Note => Box::new(NoteCmd::new(cmd, self.note_mutex.clone())),
            Type::Event => Box::new(EventCmd::new(cmd, self.event_mutex.clone())),
            Type::Exec => Box::new(ExecCmd::new(cmd)),
            _ => Box::new(NotFoundCmd::new(cmd)),
        }
    }
}
