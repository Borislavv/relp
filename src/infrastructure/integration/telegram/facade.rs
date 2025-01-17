use reqwest::Error;
use integration::telegram;
use crate::infrastructure::integration;
use integration::telegram::model::{GetUpdatesResponse, SendMessageResponse};

pub trait FacadeTrait: telegram::service::ServiceTrait + Send + Sync {}

pub struct Facade {
    service: Box<dyn telegram::service::ServiceTrait>,
}

impl Facade {
    pub fn new(service: Box<dyn telegram::service::ServiceTrait>) -> Self {
        Self { service }
    }
}

impl telegram::service::ServiceTrait for Facade {
    fn get_updates(&self, offset: i64) -> Result<GetUpdatesResponse, Error> {
        self.service.get_updates(offset)
    }
    fn send_message(&self, chat_id: u64, message: &str) -> Result<SendMessageResponse, Error> {
        self.service.send_message(chat_id, message)
    }
}

impl FacadeTrait for Facade {}