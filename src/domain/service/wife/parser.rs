use crate::app::cfg::cfg::Cfg;
use crate::domain::model::wife::Message;
use csv::ReaderBuilder;
use std::error::Error;

pub trait Parser {
    fn parse(self) -> Result<Vec<Message>, Box<dyn Error>>;
}

pub struct MessageParser {
    filepath: String,
}

impl MessageParser {
    pub fn new(cfg: Cfg) -> Self {
        Self {
            filepath: cfg.wife_filepath,
        }
    }
}

impl Parser for MessageParser {
    fn parse(self) -> Result<Vec<Message>, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(&self.filepath)?;

        let mut messages: Vec<Message> = Vec::new();

        for result in reader.deserialize() {
            match result {
                Ok(text) => messages.push(Message::new(text)),
                Err(e) => return Err(Box::new(e)),
            };
        }

        Ok(messages)
    }
}
