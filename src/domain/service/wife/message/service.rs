use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard};
use rand::Rng;
use crate::domain::error::wife::WifeMessagesVecIsEmptyError;
use crate::domain::model::wife::Message;
use crate::domain::service::wife::message::parser::MessageParser;

pub trait MessageServiceTrait: Send + Sync {
    fn get_rand(&self) -> Message;
}

pub struct MessageService {
    messages: Mutex<Vec<Message>>,
}

impl MessageService {
    pub fn new(message_parser: Arc<Box<dyn MessageParser>>) -> Result<Self, Box<dyn Error>> {
        let messages = match message_parser.parse() {
            Ok(message) => message,
            Err(error) => return Err(error),
        };

        if messages.is_empty() {
            return Err(Box::new(WifeMessagesVecIsEmptyError::new()));
        }

        Ok(Self { messages: Mutex::new(messages) })
    }
}
impl MessageServiceTrait for MessageService {
    fn get_rand(&self) -> Message {
        let vec: MutexGuard<Vec<Message>> = self.messages.lock().unwrap();

        let mut rng = rand::rng();
        let random = rng.random::<i32>() as usize;

        let m: Message = vec[random].clone();

        m
    }
}