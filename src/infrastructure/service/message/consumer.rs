use log::{error, info};
use integration::telegram;
use std::sync::mpsc::Receiver;
use crate::infrastructure::integration;
use crate::infrastructure::factory::command::Factoryer;
use crate::domain::service::executor::executor::Executor;

pub trait Consumer: Send {
    fn consume(&self, ch: Receiver<telegram::model::Message>);
}

pub struct MessageConsumer {
    executor: Box<dyn Executor>,
    factory: Box<dyn Factoryer>,
}

impl MessageConsumer {
    pub fn new(executor: Box<dyn Executor>, factory: Box<dyn Factoryer>) -> MessageConsumer {
        MessageConsumer { executor, factory }
    }
}

impl Consumer for MessageConsumer {
    fn consume(&self, msg_ch: Receiver<telegram::model::Message>) {
        for msg in msg_ch {
            match self.executor.exec(self.factory.make(msg.clone())) {
                Ok(_) => info!("Command: {} successfully executed.", msg.text),
                Err(e) => error!("Error: {} occurred while execution command: {}.", e, msg.text),
            }
        }
    }
}