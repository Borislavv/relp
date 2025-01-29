use reqwest::blocking::{Client as ReqwestClient, Response};
use reqwest::Error;
use std::time::Duration;

const TELEGRAM_API_URL: &str = "https://api.telegram.org";
const TELEGRAM_API_SEND_MESSAGE_METHOD: &str = "sendMessage";
const TELEGRAM_API_FETCH_MESSAGES_METHOD: &str = "getUpdates";

pub trait HttpClient: Send + Sync {
    fn send_message(&self, chat_id: u64, msg: &str) -> Result<Response, Error>;
    fn get_updates(&self, offset: i64) -> Result<Response, Error>;
}

pub struct Client {
    token: String,
    timeout: Duration,
}

impl Client {
    pub fn new(token: String, timeout: Duration) -> Self {
        Self { token, timeout }
    }
}

impl HttpClient for Client {
    fn send_message(&self, chat_id: u64, msg: &str) -> Result<Response, Error> {
        ReqwestClient::new()
            .post(format!(
                "{}/bot{}/{}?parse_mode=Markdown",
                TELEGRAM_API_URL, self.token, TELEGRAM_API_SEND_MESSAGE_METHOD
            ))
            .timeout(self.timeout)
            .json(&serde_json::json!({
                "chat_id": chat_id,
                "text": msg,
            }))
            .send()
    }

    fn get_updates(&self, offset: i64) -> Result<Response, Error> {
        ReqwestClient::builder()
            .timeout(self.timeout)
            .build()
            .unwrap()
            .get(format!(
                "{}/bot{}/{}?offset={}",
                TELEGRAM_API_URL, self.token, TELEGRAM_API_FETCH_MESSAGES_METHOD, offset
            ))
            .send()
    }
}
