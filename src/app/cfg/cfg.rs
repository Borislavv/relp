use std::env;
use std::time::Duration;

#[derive(Clone)]
pub struct Cfg {
    pub chat_id: u64,
    pub token: String,
    pub poll_frequency: Duration,
    pub wife_filepath: String,
    pub is_wife_mode_enabled: bool,
    pub event_loop_channel_capacity: usize,
}
impl Cfg {
    pub fn new() -> Result<Self, env::VarError> {
        let s = Self {
            chat_id: env::var("TG_CHAT_ID")
                .unwrap_or("1063099947".to_string())
                .parse::<u64>()
                .unwrap(),
            token: env::var("TG_TOKEN")
                .unwrap_or("8124548645:AAE-3yG8AZTmZRvB5Y0K8qhuVopLhsSFYE0".to_string()),
            poll_frequency: Duration::from_secs(
                env::var("TG_POLL_FREQUENCY_SEC")
                    .unwrap_or("5".to_string())
                    .parse()
                    .unwrap(),
            ),
            wife_filepath: env::var("WIFE_FILE_PATH")
                .unwrap_or("beloved_wife.csv".to_string()),
            is_wife_mode_enabled: env::var("IS_WIFE_MODE_ENABLED").unwrap_or("false".to_string())
                == "true",
            event_loop_channel_capacity: env::var("EVENT_LOOP_CHANNEL_CAPACITY")
                .unwrap_or("100".to_string())
                .parse()
                .unwrap(),
        };

        println!("Using environment variable TG_TOKEN={}", s.token);
        println!("Using environment variable TG_CHAT_ID={}", s.chat_id);
        println!(
            "Using environment variable TG_POLL_FREQUENCY_SEC={:?}",
            s.poll_frequency
        );
        println!(
            "Using environment variable IS_WIFE_MODE_ENABLED={}",
            s.is_wife_mode_enabled
        );
        println!(
            "Using environment variable EVENT_LOOP_CHANNEL_CAPACITY={}",
            s.event_loop_channel_capacity
        );

        Ok(s)
    }
}
