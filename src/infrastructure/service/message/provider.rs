use std::panic;
use log::error;
use std::sync::mpsc::Sender;
use crate::infrastructure::service;
use crate::infrastructure::integration;
use integration::telegram::dto::Message;
use service::message::poller::{LongPoller, Poller};

pub trait Provider: Send {
    fn provide(&mut self, ch: Sender<Message>);
}

impl Provider for LongPoller {
    fn provide(&mut self, ch: Sender<Message>) {
        let state = panic::catch_unwind(|| {
            self.poll(ch.clone());
        });

        match state {
            Ok(_) => (),
            Err(e) => {
                if let Some(e) = e.downcast_ref::<&str>() {
                    error!("Poll error: {}. Rerunning right now...", e);
                }

                self.provide(ch.clone())
            }
        }
    }
}