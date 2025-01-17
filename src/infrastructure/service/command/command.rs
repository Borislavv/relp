use shlex::split;
use std::cmp::Ordering;
use service::command::model;
use std::sync::{Arc, Mutex};
use crate::infrastructure::service;
use std::process::{Command as OsCmd};
use service::command::model::{Command, Exit};
use crate::infrastructure::helper::date::parse_yyyy_mm_dd_hm_from_str;

pub trait Executable {
    fn exec(&self) -> Exit;
}

pub struct Ping {}
impl Ping {
    pub fn new() -> Ping {
        Ping{}
    }
}
impl Executable for Ping {
    fn exec(&self) -> Exit {
        Exit::new(0, "pong".to_string(), "".to_string(), None)
    }
}

pub struct Cmd {
    cmd: Command
}
impl Cmd {
    pub fn new(cmd: Command) -> Cmd {
        Cmd{ cmd }
    }
}
impl Executable for Cmd {
    fn exec(&self) -> Exit {
        let cmd_parts: &mut Vec<String> = &mut split(self.cmd.str.as_str())
            .expect("Failed to parse command.");

        if cmd_parts.len() == 0 {
            return Exit::new(3, "".to_string(),
                "The command is empty, please check the send data and try again.".to_string(),
                None,
            )
        }

        let cmd_name = cmd_parts.remove(0);

        let output = OsCmd::new(cmd_name) // make a new process
            .args(cmd_parts) // passing args. into the command
            .output() // run process
            .expect("Failed to execute command.");

        Exit::new(
            output.status.code().unwrap(),
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
            Some(self.cmd.message.clone()),
        )
    }
}

pub struct Note {
    cmd: Command,
    list: Arc<Mutex<Vec<String>>>,
}
impl Note{
    pub fn new(cmd: Command, list: Arc<Mutex<Vec<String>>>) -> Note {
        Note{ cmd, list }
    }
}
impl Executable for Note {
    fn exec(&self) -> Exit {
        let msg = Some(self.cmd.message.clone());
        if self.cmd.str != String::new() {
            self.list.lock().unwrap().push(self.cmd.str.clone());
            Exit::new(0, "Successfully added.".to_string(), "".to_string(), msg)
        } else {
            Exit::new(
                0,
                self.list.lock().unwrap()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("\n"),
                "".to_string(),
                msg
            )
        }
    }
}

pub struct Event{
    cmd: Command,
    list: Arc<Mutex<Vec<model::Event>>>,
}
impl Event {
    pub fn new(cmd: Command, list: Arc<Mutex<Vec<model::Event>>>) -> Event {
        Event{ cmd, list }
    }
}
impl Executable for Event {
    fn exec(&self) -> Exit {
        let msg = Some(self.cmd.message.clone());
        if self.cmd.str != String::new() {
            let datetime = match parse_yyyy_mm_dd_hm_from_str(
                self.cmd.str.clone().as_str(),
            ) {
                Ok(date) => date,
                Err(_) => return Exit::new(
                    1, "".to_string(), "Failed to parse date.".to_string(), msg),
            };

            self.list.lock().unwrap().push(model::Event::new(self.cmd.str.clone(), datetime));

            Exit::new(0, "Successfully added.".to_string(), "".to_string(), msg)
        } else {
            self.list.lock().unwrap().sort_by(|a: &model::Event, b: &model::Event| {
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
                msg,
            )
        }
    }
}

pub struct NotFound {
    cmd: Command
}
impl NotFound {
    pub fn new(cmd: Command) -> NotFound {
        NotFound{ cmd }
    }
}
impl Executable for NotFound {
    fn exec(&self) -> Exit {
        Exit::new(2, "".to_string(),
            format!("Command `{}` not found.", self.cmd.str).to_string(), None)
    }
}