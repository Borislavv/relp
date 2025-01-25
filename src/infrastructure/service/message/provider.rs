use crate::infrastructure::integration;
use crate::infrastructure::service;
use integration::telegram::model::Message;
use service::message::poller::{LongPoller, Poller};
use std::sync::mpsc::SyncSender;

pub trait Provider: Send + Sync {
    fn provide(&mut self, ch: SyncSender<Message>);
}

impl Provider for LongPoller {
    fn provide(&mut self, ch: SyncSender<Message>) {
        self.poll(ch)
    }
}