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
        self.poll(ch);
    }
}