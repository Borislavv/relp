use service::message;
use integration::telegram;
use std::sync::mpsc::Receiver;
use crate::infrastructure::service;
use crate::infrastructure::integration;

pub trait Consumer: Send {
    fn consume(&self, ch: Receiver<telegram::dto::Message>);
}

pub struct MessageConsumer {
    handler: Box<dyn message::handler::Handler>,
}

impl MessageConsumer {
    pub fn new(handler: Box<dyn message::handler::Handler>) -> MessageConsumer {
        MessageConsumer { handler }
    }
}

impl Consumer for MessageConsumer {
    fn consume(&self, r#in: Receiver<telegram::dto::Message>) {
        for msg in r#in.iter() {
            self.handler.handle(msg);
        }
    }
}