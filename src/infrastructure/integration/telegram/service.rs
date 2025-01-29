use crate::infrastructure::integration;
use integration::telegram::http::HttpClient;
use integration::telegram::model::{GetUpdatesResponse, SendMessageResponse};
use std::error::Error;

pub trait TelegramServiceTrait: Send + Sync {
    fn get_updates(&self, offset: i64) -> Result<GetUpdatesResponse, Box<dyn Error>>;
    fn send_message(
        &self,
        chat_id: u64,
        message: &str,
    ) -> Result<SendMessageResponse, Box<dyn Error>>;
}

pub struct TelegramService {
    http_client: Box<dyn HttpClient>,
}
impl TelegramService {
    pub fn new(http_client: Box<dyn HttpClient>) -> Self {
        Self { http_client }
    }
}

impl TelegramServiceTrait for TelegramService {
    fn get_updates(&self, offset: i64) -> Result<GetUpdatesResponse, Box<dyn Error>> {
        let data = self.http_client.get_updates(offset)?.text()?;
        match serde_json::from_str(&data) {
            Ok(data) => Ok(data),
            Err(err) => {
                println!("Failed to decode getUpdates method json response: {}", data);
                Err(Box::new(err))
            }
        }
    }
    fn send_message(
        &self,
        chat_id: u64,
        message: &str,
    ) -> Result<SendMessageResponse, Box<dyn Error>> {
        let data = self.http_client.send_message(chat_id, message)?.text()?;
        match serde_json::from_str(&data) {
            Ok(data) => Ok(data),
            Err(err) => {
                println!(
                    "Failed to decode sendMessage method json response: {}",
                    data
                );
                Err(Box::new(err))
            }
        }
    }
}
