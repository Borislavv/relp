use std::panic;
use service::command::enums::Type;
use service::command::dto::Command;
use crate::infrastructure::service;
use crate::infrastructure::integration;
use integration::telegram::dto::Message;

const CMD_PREFIX: &str = "/cmd";
const NOTE_PREFIX: &str = "/note";
const EVENT_PREFIX: &str = "/event";
const PING_PREFIX: &str = "/ping";
const NOT_FOUND_PREFIX: &str = "/mirror";

pub trait Builder: Send + Sync + panic::RefUnwindSafe {
    fn build(&self, msg: Message) -> Command;
}

pub struct CommandBuilder {}

impl CommandBuilder {
    pub fn new() -> CommandBuilder {
        CommandBuilder {}
    }
}

impl Builder for CommandBuilder {
    fn build(&self, msg: Message) -> Command {
        let text = msg.text;
        let text_cloned = text.clone();
        let (r#type, prefix) = match text {
            str if str.starts_with(CMD_PREFIX) => (Type::Cmd, CMD_PREFIX),
            str if str.starts_with(NOTE_PREFIX) => (Type::Note, NOTE_PREFIX),
            str if str.starts_with(EVENT_PREFIX) => (Type::Event, EVENT_PREFIX),
            str if str.starts_with(PING_PREFIX) => (Type::Ping, PING_PREFIX),
            _ => (Type::NotFound, NOT_FOUND_PREFIX)
        };

        Command::new(text_cloned.replace(prefix, ""), r#type)
    }
}