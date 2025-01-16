use log::error;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use integration::telegram;
use std::sync::{mpsc, Arc};
use crate::infrastructure::integration;
use integration::telegram::dto::Message;

// Poller is a provider part for "provider-consumer" pattern.
pub trait Poller {
    fn poll(&self, out: mpsc::Sender<Message>);
}

pub struct LongPoller {
    freq: Duration,
    telegram: Arc<Box<dyn telegram::facade::FacadeTrait>>
}
impl LongPoller {
    pub fn new(
        freq: Duration,
        telegram: Arc<Box<dyn telegram::facade::FacadeTrait>>
    ) -> Self {
        LongPoller { freq, telegram }
    }
}
impl LongPoller {
    // returns a new offset (a last msg id + 1)
    fn offset(&self) -> i64 {
        let response = self.telegram.get_updates(0).unwrap();

        let offset: i64;
        if let Some(l) = response.result.last() {
            offset = l.update_id + 1;
        } else {
            offset = 0;
        }

        offset
    }
}
impl Poller for LongPoller {
    fn poll(&self, out: mpsc::Sender<Message>) {
        let offset = Rc::new(RefCell::new(self.offset()));

        loop {
            let offset_cloned = offset.clone();
            match self.telegram.get_updates(offset.clone().borrow().abs()) {
                Ok(r) => {
                    for u in r.result {
                        let msg: Message = if u.message.is_some() {
                            let Some(m) = u.message else {
                                panic!("Logic error, must not be here due to message already is Some.")
                            };
                            m
                        } else if u.edited_message.is_some() {
                            let Some(m) = u.edited_message else {
                                panic!("Logic error, must not be here due to edited_message already is Some.")
                            };
                            m
                        } else {
                            let json = serde_json::to_string(&u).unwrap();
                            error!("Another one unknown message type. \
                            Dump the json and check what's new up there. Data: {}", json);
                            continue;
                        };

                        out.send(msg).unwrap();
                        let mut offset_borrowed = offset_cloned.borrow_mut();
                        *offset_borrowed = u.update_id + 1
                    }
                },
                Err(e) => {
                    error!("Error getting updates: {}", e);
                }
            }

            if self.freq > Duration::from_secs(0) {
                std::thread::sleep(self.freq);
            }
        }
    }
}