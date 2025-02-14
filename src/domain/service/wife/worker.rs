use crate::domain::model::wife::Message;
use crate::infrastructure::integration::telegram::facade::TelegramFacadeTrait;
use std::sync::{Arc, Mutex};

pub trait Worker {
    fn run(&mut self);
}

pub struct WifeWorker {
    messages: Arc<Mutex<Vec<Message>>>,
    telegram: Arc<Box<dyn TelegramFacadeTrait>>,
}

impl WifeWorker {
    pub fn new(
        messages: Arc<Mutex<Vec<Message>>>,
        telegram: Arc<Box<dyn TelegramFacadeTrait>>,
    ) -> WifeWorker {
        WifeWorker { messages, telegram }
    }
}

impl Worker for WifeWorker {
    fn run(&mut self) {
        loop {}
    }
}
