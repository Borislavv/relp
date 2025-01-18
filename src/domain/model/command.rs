use shlex::split;
use std::cmp::Ordering;
use chrono::{Local, NaiveDateTime};
use std::sync::{Arc, Mutex};
use std::process::{Command as OsCmd};
use crate::infrastructure::model::command;
use crate::infrastructure::model::command::{Command, Exit};
use crate::infrastructure::helper::date::parse_yyyy_mm_dd_hm_from_str;

pub trait Executable {
    fn exec(&self) -> Exit;
}

pub struct PingCmd {}
impl PingCmd {
    pub fn new() -> PingCmd {
        PingCmd {}
    }
}
impl Executable for PingCmd {
    fn exec(&self) -> Exit {
        Exit::new(0, "pong".to_string(), "".to_string(), None)
    }
}

pub struct ExecCmd {
    cmd: Command
}
impl ExecCmd {
    pub fn new(cmd: Command) -> ExecCmd {
        ExecCmd { cmd }
    }
}
impl Executable for ExecCmd {
    fn exec(&self) -> Exit {
        let cmd_parts: &mut Vec<String> = &mut split(self.cmd.str.as_str())
            .expect("Failed to split str command.");

        if cmd_parts.len() == 0 {
            return Exit::new(3, "".to_string(),
                "The command is empty, please check the send data and try again.".to_string(),
                None,
            )
        }

        let cmd_name = cmd_parts.remove(0);
        let output = OsCmd::new(cmd_name) // make a new process
            .args(cmd_parts) // passing args. into the command
            .output(); // run process;

        match output {
            Ok(output) => {
                Exit::new(
                    output.status.code().unwrap(),
                    String::from_utf8(output.stdout).unwrap(),
                    String::from_utf8(output.stderr).unwrap(),
                    Some(self.cmd.message.clone()),
                )
            },
            Err(error) => {
                Exit::new(
                    1,
                    "".to_string(),
                    error.to_string(),
                    Some(self.cmd.message.clone()),
                )
            }
        }
    }
}

pub struct NoteCmd {
    cmd: Command,
    list: Arc<Mutex<Vec<Note>>>,
}
impl NoteCmd {
    pub fn new(cmd: Command, list: Arc<Mutex<Vec<Note>>>) -> NoteCmd {
        NoteCmd { cmd, list }
    }
}
impl Executable for NoteCmd {
    fn exec(&self) -> Exit {
        let msg = Some(self.cmd.message.clone());
        if self.cmd.str != String::new() {
            self.list.lock().unwrap().push(Note::new(self.cmd.str.clone()));
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

pub struct EventCmd {
    cmd: Command,
    list: Arc<Mutex<Vec<Event>>>,
}
impl EventCmd {
    pub fn new(cmd: Command, list: Arc<Mutex<Vec<Event>>>) -> EventCmd {
        EventCmd { cmd, list }
    }
}
impl Executable for EventCmd {
    fn exec(&self) -> Exit {
        let msg = Some(self.cmd.message.clone());

        if self.cmd.str != String::new() {
            let datetime = match parse_yyyy_mm_dd_hm_from_str(
                self.cmd.str.clone().as_str(),
            ) {
                Ok(date) => date,
                Err(_) => return Exit::new(1, "".to_string(), "Failed to parse date.".to_string(), msg),
            };

            self.list.lock().unwrap().push(Event::new(self.cmd.str.clone(), datetime));

            let between = datetime.signed_duration_since(Local::now().naive_local());

            Exit::new(
                0,
                format!("[{}] Successfully added and will be triggerred in {} weeks {} days \
                {} hours {} minutes {} seconds.", Local::now().naive_local().format("%Y-%m-%dT%H:%M"),
                between.num_weeks(), between .num_days(), between.num_hours(),
                between.num_minutes(), between.num_seconds()),
                "".to_string(),
                msg,
            )
        } else {
            self.list.lock().unwrap().sort_by(|a: &Event, b: &Event| {
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

pub struct NotFoundCmd {
    cmd: Command
}
impl NotFoundCmd {
    pub fn new(cmd: Command) -> NotFoundCmd {
        NotFoundCmd { cmd }
    }
}
impl Executable for NotFoundCmd {
    fn exec(&self) -> Exit {
        Exit::new(2, "".to_string(),
            format!("Command `{}` not found.", self.cmd.str).to_string(), None)
    }
}

pub struct Note {
    pub text: String,
}
impl Note {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[derive(Clone, Default)]
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
        write!(f, "{}", self.text)
    }
}