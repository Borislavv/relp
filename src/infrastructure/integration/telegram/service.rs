use reqwest::Error;
use crate::infrastructure::integration;
use integration::telegram::http::HttpClient;
use integration::telegram::dto::{GetUpdatesResponse, SendMessageResponse};

pub trait ServiceTrait: Send + Sync {
    fn get_updates(&self, offset: i64) -> Result<GetUpdatesResponse, Error>;
    fn send_message(&self, chat_id: u64, message: &str) -> Result<SendMessageResponse, Error>;
}

pub struct Service {
    http_client: Box<dyn HttpClient>
}
impl Service {
    pub fn new(http_client: Box<dyn HttpClient>) -> Self {
        Self { http_client }
    }
}

impl ServiceTrait for Service {
    fn get_updates(&self, offset: i64) -> Result<GetUpdatesResponse, Error> {
        let data = self.http_client.get_updates(offset)?.text()?;
        let resp: GetUpdatesResponse = serde_json::from_str(&data).unwrap();
        Ok(resp)
    }
    fn send_message(&self, chat_id: u64, message: &str) -> Result<SendMessageResponse, Error> {
        let data = self.http_client.send_message(chat_id, message)?.text()?;
        let resp: SendMessageResponse = serde_json::from_str(&data).unwrap();
        Ok(resp)
    }
}