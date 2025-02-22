use crate::domain::model;
use crate::domain::model::event::ExecutableEvent;
use crate::domain::r#enum::event::Repeat;
use crate::infrastructure::helper::date::parse_yyyy_mm_dd_hm_from_str;
use crate::infrastructure::model::command::{Command, Exit};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use shlex::split;
use std::cmp::Ordering;
use std::ops::Add;
use std::process::Command as OsCmd;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use crate::domain::service::wife::message::service::MessageServiceTrait;

pub trait Executable {
    fn exec(&self) -> Exit;
}

pub struct PingCmd {
    cmd: Command,
    atm: AtomicI64,
}
impl PingCmd {
    pub fn new(cmd: Command) -> PingCmd {
        PingCmd {
            cmd,
            atm: AtomicI64::new(3),
        }
    }
}
impl Executable for PingCmd {
    fn exec(&self) -> Exit {
        let remaining_repeats = self.atm.fetch_sub(1, SeqCst);

        let mut msg = "pong".to_string();
        if remaining_repeats % 2 == 0 {
            msg = "ping".to_string();
        }

        Exit::new(0, msg, "".to_string(), None)
    }
}
impl model::event::Event for PingCmd {
    fn name(&self) -> String {
        self.cmd.str.clone()
    }
    fn is_ready(&self) -> bool {
        true
    }
    fn repeats(&self) -> Repeat {
        Repeat::Times(self.atm.load(SeqCst))
    }
    fn from_self(&self) -> Self {
        Self {
            cmd: self.cmd.clone(),
            atm: AtomicI64::new(self.atm.load(SeqCst)),
        }
    }
}
impl ExecutableEvent for PingCmd {
    fn sender(&self) -> Option<Arc<Sender<Arc<Box<dyn ExecutableEvent>>>>> {
        None
    }
}

pub struct WifeMessageCmd {
    postfix: String,
    service: Arc<Box<dyn MessageServiceTrait>>,
    date: NaiveDateTime
}
impl WifeMessageCmd {
    pub fn new(date: NaiveDateTime, postfix: String, service: Arc<Box<dyn MessageServiceTrait>>) -> WifeMessageCmd {
        WifeMessageCmd { postfix, service, date }
    }
}
impl Executable for WifeMessageCmd {
    fn exec(&self) -> Exit {
        Exit::new(0, self.service.get_rand().text + self.postfix.as_str(), "".to_string(), None)
    }
}
impl model::event::Event for WifeMessageCmd {
    fn name(&self) -> String {
       "Beloved".to_string()
    }
    fn is_ready(&self) -> bool {
        let now = Local::now().naive_local();
        if self.date.day() == now.day() && now.hour() >= self.date.hour() {
            return true;
        }
        false
    }
    fn repeats(&self) -> Repeat {
        Repeat::Always
    }
    fn from_self(&self) -> Self {
        Self {
            postfix: self.postfix.clone(),
            service: self.service.clone(),
            date: self.date + Duration::days(1),
        }
    }
}
impl ExecutableEvent for WifeMessageCmd {
    fn sender(&self) -> Option<Arc<Sender<Arc<Box<dyn ExecutableEvent>>>>> {
        None
    }
}

pub struct ExecCmd {
    cmd: Command,
}
impl ExecCmd {
    pub fn new(cmd: Command) -> ExecCmd {
        ExecCmd { cmd }
    }
}
impl Executable for ExecCmd {
    fn exec(&self) -> Exit {
        let cmd_parts: &mut Vec<String> =
            &mut split(self.cmd.str.as_str()).expect("Failed to split str command.");

        if cmd_parts.len() == 0 {
            return Exit::new(
                3,
                "".to_string(),
                "The command is empty, please check the send data and try again.".to_string(),
                None,
            );
        }

        let cmd_name = cmd_parts.remove(0);
        let output = OsCmd::new(cmd_name) // make a new process
            .args(cmd_parts) // passing args. into the command
            .output(); // run process;

        match output {
            Ok(output) => Exit::new(
                output.status.code().unwrap(),
                String::from_utf8(output.stdout).unwrap(),
                String::from_utf8(output.stderr).unwrap(),
                Some(self.cmd.message.clone()),
            ),
            Err(error) => Exit::new(
                1,
                "".to_string(),
                error.to_string(),
                Some(self.cmd.message.clone()),
            ),
        }
    }
}
impl model::event::Event for ExecCmd {
    fn name(&self) -> String {
        self.cmd.str.clone()
    }
    fn is_ready(&self) -> bool {
        true
    }
    fn repeats(&self) -> Repeat {
        Repeat::Once
    }
    fn from_self(&self) -> Self {
        Self {
            cmd: self.cmd.clone(),
        }
    }
}
impl ExecutableEvent for ExecCmd {
    fn sender(&self) -> Option<Arc<Sender<Arc<Box<dyn ExecutableEvent>>>>> {
        None
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
            self.list
                .lock()
                .unwrap()
                .push(Note::new(self.cmd.str.clone()));
            Exit::new(0, "Successfully added.".to_string(), "".to_string(), msg)
        } else {
            Exit::new(
                0,
                self.list
                    .lock()
                    .unwrap()
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
impl model::event::Event for NoteCmd {
    fn name(&self) -> String {
        self.cmd.str.clone()
    }
    fn is_ready(&self) -> bool {
        true
    }
    fn repeats(&self) -> Repeat {
        Repeat::Once
    }
    fn from_self(&self) -> Self {
        Self {
            cmd: self.cmd.clone(),
            list: self.list.clone(),
        }
    }
}
impl ExecutableEvent for NoteCmd {
    fn sender(&self) -> Option<Arc<Sender<Arc<Box<dyn ExecutableEvent>>>>> {
        None
    }
}

pub struct EventCmd {
    cmd: Command,
    list: Arc<Mutex<Vec<Event>>>,
    date: NaiveDateTime,
    exit: Option<Exit>,
}
impl EventCmd {
    pub fn new(cmd: Command, list: Arc<Mutex<Vec<Event>>>) -> EventCmd {
        EventCmd {
            cmd: cmd.clone(),
            list: list.clone(),
            date: match parse_yyyy_mm_dd_hm_from_str(cmd.str.clone().as_str()) {
                Ok(datetime) => datetime,
                Err(err) => {
                    let exit = Some(Exit::new(
                        1,
                        "".to_string(),
                        err.to_string(),
                        Some(cmd.message.clone()),
                    ));
                    return EventCmd {
                        cmd,
                        list,
                        date: Local::now().naive_local(),
                        exit,
                    };
                }
            },
            exit: None,
        }
    }
}
impl Executable for EventCmd {
    fn exec(&self) -> Exit {
        let msg = Some(self.cmd.message.clone());

        if self.cmd.str != String::new() {
            let datetime = match parse_yyyy_mm_dd_hm_from_str(self.cmd.str.clone().as_str()) {
                Ok(datetime) => datetime,
                Err(err) => return Exit::new(1, "".to_string(), err.to_string(), msg),
            };

            self.list
                .lock()
                .unwrap()
                .push(Event::new(self.cmd.str.clone(), datetime));

            let between_dates = datetime.signed_duration_since(Local::now().naive_local());
            let remaining_hours = between_dates.num_hours() - (between_dates.num_days() * 24);
            let remaining_minutes = between_dates.num_minutes() - (between_dates.num_hours() * 60);
            let remaining_seconds =
                between_dates.num_seconds() - (between_dates.num_minutes() * 60);

            let days_string = match between_dates.num_days() {
                0 => "".to_string(),
                _ => format!(" {} days", between_dates.num_days()),
            };

            let hours_string = match remaining_hours {
                0 => "".to_string(),
                _ => format!(" {} hours", remaining_hours),
            };

            let minutes_string = match remaining_minutes {
                0 => "".to_string(),
                _ => format!(" {} minutes", remaining_minutes),
            };

            let seconds_string = match remaining_seconds {
                0 => "".to_string(),
                _ => format!(" {} seconds", remaining_seconds),
            };

            Exit::new(
                0,
                format!(
                    "[{}] Successfully added and will be triggerred in{}{}{}{}.",
                    Local::now().naive_local().format("%Y-%m-%dT%H:%M"),
                    days_string,
                    hours_string,
                    minutes_string,
                    seconds_string,
                ),
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
                self.list
                    .lock()
                    .unwrap()
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
impl model::event::Event for EventCmd {
    fn name(&self) -> String {
        self.cmd.str.clone()
    }
    fn is_ready(&self) -> bool {
        self.date < Local::now().naive_local()
    }
    fn repeats(&self) -> Repeat {
        Repeat::Once
    }
    fn from_self(&self) -> Self {
        Self {
            cmd: self.cmd.clone(),
            list: self.list.clone(),
            date: self.date,
            exit: None,
        }
    }
}
impl ExecutableEvent for EventCmd {
    fn sender(&self) -> Option<Arc<Sender<Arc<Box<dyn ExecutableEvent>>>>> {
        None
    }
}

pub struct NotFoundCmd {
    cmd: Command,
}
impl NotFoundCmd {
    pub fn new(cmd: Command) -> NotFoundCmd {
        NotFoundCmd { cmd }
    }
}
impl Executable for NotFoundCmd {
    fn exec(&self) -> Exit {
        Exit::new(
            2,
            "".to_string(),
            format!("Command `{}` not found.", self.cmd.str).to_string(),
            None,
        )
    }
}
impl model::event::Event for NotFoundCmd {
    fn name(&self) -> String {
        self.cmd.str.clone()
    }
    fn is_ready(&self) -> bool {
        true
    }
    fn repeats(&self) -> Repeat {
        Repeat::Once
    }
    fn from_self(&self) -> Self {
        Self {
            cmd: self.cmd.clone(),
        }
    }
}
impl ExecutableEvent for NotFoundCmd {
    fn sender(&self) -> Option<Arc<Sender<Arc<Box<dyn ExecutableEvent>>>>> {
        None
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
    pub date: NaiveDateTime,
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
