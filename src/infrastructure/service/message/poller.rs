use crate::app::model::state::State;
use crate::domain::error::message::UnknownMessageTypeError;
use crate::infrastructure::integration;
use integration::telegram;
use integration::telegram::model::Message;
use std::sync::{mpsc, Arc};
use std::time::Duration;

// Poller is a provider part for "provider-consumer" pattern.
pub trait Poller {
    fn poll(&self, out: mpsc::SyncSender<Message>);
}

pub struct LongPoller {
    freq: Duration,
    state: Arc<Box<dyn State>>,
    telegram: Arc<Box<dyn telegram::facade::TelegramFacadeTrait>>,
}
impl LongPoller {
    pub fn new(
        freq: Duration,
        state: Arc<Box<dyn State>>,
        telegram: Arc<Box<dyn telegram::facade::TelegramFacadeTrait>>,
    ) -> Self {
        LongPoller {
            freq,
            state,
            telegram,
        }
    }
}
impl LongPoller {
    // returns a new offset (a last msg id + 1)
    fn query_offset(&self) -> i64 {
        let response = match self.telegram.get_updates(0) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}", e);
                return 0;
            }
        };

        let offset: i64;
        if let Some(l) = response.result.last() {
            offset = l.update_id + 1;
        } else {
            offset = 0;
        }

        offset
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
}
impl Poller for LongPoller {
    fn poll(&self, out: mpsc::SyncSender<Message>) {
        if self.state.is_closed() {
            return;
        }

        let mut offset = self.query_offset();

        loop {
            match self.telegram.get_updates(offset.clone()) {
                Ok(r) => {
                    for update in r.result {
                        // joining of message and edited message
                        // (will be selected just one of which is not None)
                        let msg = Self::extract_msg(update.message, update.edited_message).unwrap();

                        // send the message to the other side
                        out.send(msg).unwrap();

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
