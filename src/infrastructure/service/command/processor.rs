use std::sync::{Arc, Mutex};
use service::command::enums;
use crate::infrastructure::service;
use service::command::{command, dto};
use service::command::dto::{Command, Exit};

pub trait Processor: Send + Sync {
    fn process(&self, cmd: Command) -> Exit;
}

pub struct CommandProcessor {
    note_mutex: Arc<Mutex<Vec<String>>>,
    event_mutex: Arc<Mutex<Vec<dto::Event>>>,
}
impl CommandProcessor {
    pub fn new() -> CommandProcessor {
        CommandProcessor {
            note_mutex: Arc::new(Mutex::new(vec![])),
            event_mutex: Arc::new(Mutex::new(vec![])),
        }
    }
}
impl Processor for CommandProcessor {
    fn process(&self, cmd: Command) -> Exit {
        let executable: Box<dyn command::Executable> = match cmd.r#type {
            enums::Type::Ping => Box::new(command::Ping::new()),
            enums::Type::Note => Box::new(command::Note::new(self.note_mutex.clone())),
            enums::Type::Event => Box::new(command::Event::new(self.event_mutex.clone())),
            enums::Type::Cmd => Box::new(command::Cmd::new()),
            _ => Box::new(command::NotFound::new()),
        };

        executable.exec(cmd)
    }
}