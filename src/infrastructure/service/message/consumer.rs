use crate::app::model::state::State;
use crate::domain::factory::command::Factoryer;
use crate::domain::model::event::ExecutableEvent;
use crate::domain::service::event::r#loop::EventLoop;
use crate::domain::service::executor::executor::Executor;
use crate::infrastructure::integration;
use integration::telegram;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

pub trait Consumer: Send + Sync {
    fn consume(&self, ch: Receiver<telegram::model::Message>);
}

pub struct MessageConsumer {
    factory: Box<dyn Factoryer>,
    state: Arc<Box<dyn State>>,
    event_loop: Arc<Box<dyn EventLoop>>,
}

impl MessageConsumer {
    pub fn new(
        factory: Box<dyn Factoryer>,
        state: Arc<Box<dyn State>>,
        event_loop: Arc<Box<dyn EventLoop>>,
    ) -> MessageConsumer {
        MessageConsumer {
            factory,
            state,
            event_loop,
        }
    }
}

impl Consumer for MessageConsumer {
    fn consume(&self, msg_ch: Receiver<telegram::model::Message>) {
        for msg in msg_ch {
            if self.state.is_closed() {
                return;
            }

            let event: Arc<Box<dyn ExecutableEvent>> = Arc::new(self.factory.make(msg.clone()));

            self.event_loop.clone().add_event(event);
        }
    }
}
