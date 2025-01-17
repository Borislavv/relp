use std::env;
use std::time::Duration;
use log::info;

#[derive(Clone)]
pub struct Cfg {
    pub chat_id: u64,
    pub token: String,
    pub poll_frequency: Duration,
}
impl Cfg {
    pub fn new() -> Result<Self, env::VarError> {
        let s = Self {
            token: env::var("TG_TOKEN").unwrap_or("8124548645:AAE-3yG8AZTmZRvB5Y0K8qhuVopLhsSFYE0".to_string()),
            chat_id: env::var("TG_CHAT_ID").unwrap_or("1063099947".to_string()).parse::<u64>().unwrap(),
            poll_frequency: Duration::from_secs(env::var("TG_POLL_FREQUENCY_SEC").unwrap_or("5".to_string()).parse().unwrap()),
        };

        info!("Using environment variable TG_TOKEN={}", s.token);
        info!("Using environment variable TCH_CHAT_ID={}", s.chat_id);
        info!("Using environment variable TG_TOKEN={}", s.token);

        Ok(s)
    }
}