use std::sync::Arc;
use log::{error, info};
use integration::telegram;
use std::sync::mpsc::Receiver;
use crate::app::model::state::State;
use crate::infrastructure::integration;
use crate::domain::factory::command::Factoryer;
use crate::domain::service::executor::executor::Executor;

pub trait Consumer: Send + Sync {
    fn consume(&self, ch: Receiver<telegram::model::Message>);
}

pub struct MessageConsumer {
    executor: Box<dyn Executor>,
    factory: Box<dyn Factoryer>,
    state: Arc<Box<dyn State>>,
}

impl MessageConsumer {
    pub fn new(executor: Box<dyn Executor>, factory: Box<dyn Factoryer>, state: Arc<Box<dyn State>>,) -> MessageConsumer {
        MessageConsumer { executor, factory, state }
    }
}

impl Consumer for MessageConsumer {
    fn consume(&self, msg_ch: Receiver<telegram::model::Message>) {
        for msg in msg_ch {
            if self.state.is_closed() {
                return;
            }

            match self.executor.exec(self.factory.make(msg.clone())) {
                Ok(_) => info!("Command: {} successfully executed.", msg.text),
                Err(e) => error!("Error: {} occurred while execution command: {}.", e, msg.text),
            }
        }
    }
}