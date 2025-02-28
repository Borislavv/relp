use std::ops::Add;
use crate::app::model::state::State;
use crate::domain::error::message::{OffsetFetchError, UnknownMessageTypeError};
use crate::infrastructure::integration;
use integration::telegram;
use integration::telegram::model::Message;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use chrono::Local;
use crate::app::cfg::cfg::Cfg;

// Poller is a provider part for "provider-consumer" pattern.
pub trait Poller {
    fn poll(&self, out: mpsc::SyncSender<Message>);
}

pub struct LongPoller {
    cfg: Cfg,
    freq: Duration,
    state: Arc<Box<dyn State>>,
    telegram: Arc<Box<dyn telegram::facade::TelegramFacadeTrait>>,
}
impl LongPoller {
    pub fn new(
        cfg: Cfg,
        freq: Duration,
        state: Arc<Box<dyn State>>,
        telegram: Arc<Box<dyn telegram::facade::TelegramFacadeTrait>>,
    ) -> Self {
        LongPoller {
            cfg,
            freq,
            state,
            telegram,
        }
    }
}
impl LongPoller {
    // returns a new offset or propagated a panic!
    fn get_offset_with_retries(&self) -> i64 {
        let threshold = Local::now().naive_local().add(chrono::Duration::seconds(self.freq.as_secs() as i64));
        while Local::now().naive_local() < threshold {
            match self.query_offset() {
                Ok(offset) => {
                    println!("Offset has been received, start processing messages...");
                    return offset;
                },
                Err(e) => eprintln!("{}", e)
            };

            std::thread::sleep(Duration::from_secs(1));
        }

        panic!("Failed to query offset, timeout exceeded.");
    }
    // returns a new offset (a last msg id + 1)
    fn query_offset(&self) -> Result<i64, OffsetFetchError> {
        let response = match self.telegram.get_updates(0) {
            Ok(response) => response,
            Err(error) => return Err(OffsetFetchError::new(Some(error))),
        };

        if !response.ok {
            return Err(OffsetFetchError::new(None));
        }

        if response.result.len() == 0 {
            return Ok(0);
        }

        if let Some(l) = response.result.last() {
            return Ok(l.update_id + 1);
        } else {
            let json = serde_json::to_string(&response.result).unwrap();
            eprintln!("Unknown response from telegram: {}.", json);
        }

        Err(OffsetFetchError::new(None))
    }
    fn extract_msg(
        msg: Option<Message>,
        edited_msg: Option<Message>,
    ) -> Result<Message, UnknownMessageTypeError> {
        if msg.is_some() {
            let Some(message) = msg else {
                panic!("Logic error, must not be here due to message already is Some.")
            };
            Ok(message)
        } else if edited_msg.is_some() {
            let Some(message) = edited_msg else {
                panic!("Logic error, must not be here due to edited_message already is Some.")
            };
            Ok(message)
        } else {
            println!(
                "Another one unknown message type. Dump the json and check what's new up there."
            );
            Err(UnknownMessageTypeError::new())
        }
    }
    fn is_must_be_skipped(
        &self,
        msg: Option<Message>,
        edited_msg: Option<Message>,
    ) -> bool {
        if let Some(message) = msg {
            if self.cfg.chat_id != u64::try_from(message.chat.id).unwrap() {
                return true;
            }
        }
        if let Some(message) = edited_msg {
            if self.cfg.chat_id != u64::try_from(message.chat.id).unwrap() {
                return true;
            }
        }
        false
    }
}
impl Poller for LongPoller {
    fn poll(&self, out: mpsc::SyncSender<Message>) {
        if self.state.is_closed() {
            return;
        }

        let mut offset = self.get_offset_with_retries();

        loop {
            match self.telegram.get_updates(offset.clone()) {
                Ok(r) => {
                    for update in r.result {
                        // handle messages just from myself
                        if !self.is_must_be_skipped(update.message.clone(), update.edited_message.clone()) {
                            // joining of message and edited message
                            // (will be selected just one of which is not None)
                            let msg = Self::extract_msg(update.message, update.edited_message).unwrap();

                            // send the message to the other side
                            out.send(msg).unwrap();
                        }

                        // calculate a new offset
                        offset = update.update_id + 1;
                    }
                }
                Err(e) => println!("Error getting updates: {}", e),
            };

            if self.freq > Duration::from_secs(0) {
                std::thread::sleep(self.freq);
            }
        }
    }
}
