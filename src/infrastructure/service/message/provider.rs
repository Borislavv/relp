use std::sync::mpsc::SyncSender;
use crate::infrastructure::service;
use crate::infrastructure::integration;
use integration::telegram::model::Message;
use service::message::poller::{LongPoller, Poller};

pub trait Provider: Send {
    fn provide(&mut self, ch: SyncSender<Message>);
}

impl Provider for LongPoller {
    fn provide(&mut self, ch: SyncSender<Message>) {
        self.poll(ch.clone());
    }
}