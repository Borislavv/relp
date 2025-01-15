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
        Ok(Self {
            token: env::var("TG_TOKEN")?,
            chat_id: env::var("TG_CHAT_ID")?.parse::<u64>().unwrap(),
            poll_frequency: Duration::from_secs(env::var("TG_POLL_FREQUENCY_SEC")?.parse().unwrap()),
        })
    }
}