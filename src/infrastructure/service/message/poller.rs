use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;
use integration::telegram;
use crate::infrastructure::integration;
use integration::telegram::dto::Message;

// Poller is a provider part for "provider-consumer" pattern.
pub trait Poller {
    fn poll(&mut self, out: mpsc::Sender<Message>);
}

pub struct LongPoller {
    offset: AtomicI64,
    freq: Duration,
    telegram: Arc<Box<dyn telegram::facade::FacadeTrait>>
}
impl LongPoller {
    pub fn new(
        offset: Option<i64>,
        freq: Duration,
        telegram: Arc<Box<dyn telegram::facade::FacadeTrait>>
    ) -> Self {
        let offset: AtomicI64 = AtomicI64::new(offset.unwrap_or(0));
        LongPoller { offset, freq, telegram }
    }
}
impl Poller for LongPoller {
    fn poll(&mut self, out: mpsc::Sender<Message>) {
        loop {
            match self.telegram.get_updates(self.offset.load(Ordering::SeqCst)) {
                Ok(r) => {
                    for u in r.result {
                        let msg: Message = if u.message.is_some() {
                            u.message.unwrap()
                        } else if u.edited_message.is_some() {
                            u.edited_message.unwrap()
                        } else {
                            panic!("Another one unknown message type. \
                            Dump the json and check what's new up there.")
                        };

                        out.send(msg).unwrap();
                        self.offset.store(u.update_id + 1, Ordering::SeqCst);
                    }
                },
                Err(e) => {
                    println!("Error getting updates: {}", e);
                }
            }

            if self.freq > Duration::from_secs(0) {
                std::thread::sleep(self.freq);
            }
        }
    }
}