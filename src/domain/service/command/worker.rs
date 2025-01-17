use chrono::Local;
use std::sync::{Arc, Mutex};
use crate::app::cfg::cfg::Cfg;
use crate::app::model::state::State;
use crate::domain::model::command::Event;
use crate::infrastructure::integration::telegram::facade::TelegramFacadeTrait;

pub trait Worker: Send + Sync {
    fn serve(&self);
}

pub struct CommandWorker {
    cfg: Cfg,
    state: Arc<Box<dyn State>>,
    event_mutex: Arc<Mutex<Vec<Event>>>,
    telegram: Arc<Box<dyn TelegramFacadeTrait>>,
}

impl CommandWorker {
    pub fn new(
        cfg: Cfg,
        state: Arc<Box<dyn State>>,
        event_mutex: Arc<Mutex<Vec<Event>>>,
        telegram: Arc<Box<dyn TelegramFacadeTrait>>) -> Self
    {
        Self { cfg, state, event_mutex, telegram }
    }
}

impl Worker for CommandWorker {
    fn serve(&self) {
        loop {
            if self.state.is_closed() {
                return;
            }

            match self.event_mutex.lock() {
                Ok(mut events) => {
                    let mut events_vec: Vec<Event> = vec![Default::default(); events.len()];
                    events_vec.clone_from_slice(events.as_slice());
                    for (key, event) in events_vec.iter().enumerate() {
                        if Local::now().naive_local() > event.date {
                            if let Err(e) = self.telegram.send_message(self.cfg.chat_id, event.to_string().as_str()) {
                                println!("domain::service::command:worker: failed to send message while serve Event. Error: {}.", e);
                                println!("domain::service::command:worker: failed to send message while serve Event. Error: {}.", e);
                            } else {
                                events.remove(key);
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("domain::service::command:worker: failed to get event_mutex lock. Error: {}", e);
                    println!("domain::service::command:worker: failed to get event_mutex lock. Error: {}", e);
                }
            };

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }
}