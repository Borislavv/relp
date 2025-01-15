use shlex::split;
use std::cmp::Ordering;
use service::command::dto;
use std::sync::{Arc, Mutex};
use crate::infrastructure::service;
use std::process::{Command as OsCmd};
use service::command::dto::{Command, Exit};

pub trait Executable {
    fn exec(&self, cmd: Command) -> Exit;
}

pub struct Ping {}
impl Ping {
    pub fn new() -> Ping {
        Ping{}
    }
}
impl Executable for Ping {
    fn exec(&self, _: Command) -> Exit {
        Exit::new(0, "pong".to_string(), "".to_string())
    }
}

pub struct Cmd {}
impl Cmd {
    pub fn new() -> Cmd {
        Cmd{}
    }
}
impl Executable for Cmd {
    fn exec(&self, cmd: Command) -> Exit {
        let cmd_parts: &mut Vec<String> = &mut split(cmd.str.as_str())
            .expect("Failed to parse command.");
        let cmd_name = cmd_parts.remove(0);

        let output = OsCmd::new(cmd_name) // make a new process
            .args(cmd_parts) // passing args. into the command
            .output() // run process
            .expect("Failed to execute command.");

        Exit::new(
            output.status.code().unwrap(),
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
        )
    }
}

pub struct Note {
    list: Arc<Mutex<Vec<String>>>,
}
impl Note{
    pub fn new(list: Arc<Mutex<Vec<String>>>) -> Note {
        Note{ list }
    }
}
impl Executable for Note {
    fn exec(&self, cmd: Command) -> Exit {
        if cmd.str != String::new() {
            self.list.lock().unwrap().push(cmd.str.clone());
            Exit::new(0, "Successfully added.".to_string(), "".to_string())
        } else {
            Exit::new(
                0,
                self.list.lock().unwrap()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("\n"),
                "".to_string(),
            )
        }
    }
}

pub struct Event{
    list: Arc<Mutex<Vec<dto::Event>>>,
}
impl Event {
    pub fn new(list: Arc<Mutex<Vec<dto::Event>>>) -> Event {
        Event{ list }
    }
}
impl Executable for Event {
    fn exec(&self, cmd: Command) -> Exit {
        if cmd.str != String::new() {
            let datetime = match chrono::NaiveDateTime::parse_from_str(
                cmd.str.clone().as_str(),
                "%Y-%m-%d %H:%M"
            ) {
                Ok(date) => date,
                Err(e) => {
                    return Exit::new(
                        1,
                        "".to_string(),
                        format!("Failed to parse date. Error: {}.", e).to_string(),
                    );
                },
            };

            self.list.lock().unwrap().push(dto::Event::new(cmd.str, datetime));

            Exit::new(0, "Successfully added.".to_string(), "".to_string())
        } else {
            self.list.lock().unwrap().sort_by(|a: &dto::Event, b: &dto::Event| {
                if a.date.ge(&b.date) {
                    Ordering::Greater
                } else if a.date.le(&b.date) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });

            Exit::new(
                0,
                self.list.lock().unwrap()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("\n"),
                "".to_string(),
            )
        }
    }
}

pub struct NotFound {}
impl NotFound {
    pub fn new() -> NotFound {
        NotFound{}
    }
}
impl Executable for NotFound {
    fn exec(&self, cmd: Command) -> Exit {
        Exit::new(2, "".to_string(), format!("Command `{}` not found.", cmd.str).to_string())
    }
}