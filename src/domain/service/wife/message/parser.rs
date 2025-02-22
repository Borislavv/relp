use crate::app::cfg::cfg::Cfg;
use crate::domain::model::wife::Message;
use csv::ReaderBuilder;
use std::error::Error;

pub trait MessageParser: Send + Sync {
    fn parse(&self) -> Result<Vec<Message>, Box<dyn Error>>;
}

pub struct CsvParser {
    filepath: String,
}

impl CsvParser {
    pub fn new(cfg: Cfg) -> Self {
        Self {
            filepath: cfg.wife_filepath,
        }
    }
}

impl MessageParser for CsvParser {
    fn parse(&self) -> Result<Vec<Message>, Box<dyn Error>> {
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
