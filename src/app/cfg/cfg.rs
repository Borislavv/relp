use std::env;
use std::time::Duration;

#[derive(Clone)]
pub struct Cfg {
    pub chat_id: u64,
    pub token: String,
    pub poll_frequency: Duration,
}
impl Cfg {
    pub fn new() -> Result<Self, env::VarError> {
        let s = Self {
            token: env::var("TG_TOKEN").unwrap_or("telegram_token".to_string()),
            chat_id: env::var("TG_CHAT_ID").unwrap_or("123456789".to_string()).parse::<u64>()
                .unwrap(),
            poll_frequency: Duration::from_secs(env::var("TG_POLL_FREQUENCY_SEC").unwrap_or("5".to_string()).parse().unwrap()),
        };

        println!("Using environment variable TG_TOKEN={}", s.token);
        println!("Using environment variable TCH_CHAT_ID={}", s.chat_id);
        println!("Using environment variable TG_TOKEN={}", s.token);

        Ok(s)
    }
}