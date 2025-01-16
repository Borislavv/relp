use crate::infrastructure::service;
use crate::infrastructure::integration;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use integration::telegram::dto::Message;
use service::message::consumer::Consumer;
use service::message::provider::Provider;

pub trait Facade: Provider + Consumer {}

pub struct MessageFacade {
    provider: Box<dyn Provider>,
    consumer: Box<dyn Consumer>,
}
impl MessageFacade {
    pub fn new(provider: Box<dyn Provider>, consumer: Box<dyn Consumer>) -> Self {
        MessageFacade { provider, consumer }
    }
}
impl Provider for MessageFacade {
    fn provide(&mut self, ch: SyncSender<Message>) {
        self.provider.provide(ch);
    }
}
impl Consumer for MessageFacade {
    fn consume(&self, ch: Receiver<Message>) {
        self.consumer.consume(ch);
    }
}