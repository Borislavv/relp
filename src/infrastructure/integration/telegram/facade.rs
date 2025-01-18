use std::error::Error;
use integration::telegram;
use crate::infrastructure::integration;
use integration::telegram::model::{GetUpdatesResponse, SendMessageResponse};

pub trait TelegramFacadeTrait: telegram::service::TelegramServiceTrait + Send + Sync {}

pub struct TelegramFacade {
    service: Box<dyn telegram::service::TelegramServiceTrait>,
}

impl TelegramFacade {
    pub fn new(service: Box<dyn telegram::service::TelegramServiceTrait>) -> Self {
        Self { service }
    }
}

impl telegram::service::TelegramServiceTrait for TelegramFacade {
    fn get_updates(&self, offset: i64) -> Result<GetUpdatesResponse, Box<dyn Error>> {
        self.service.get_updates(offset)
    }
    fn send_message(&self, chat_id: u64, message: &str) -> Result<SendMessageResponse, Box<dyn Error>> {
        self.service.send_message(chat_id, message)
    }
}

impl TelegramFacadeTrait for TelegramFacade {}