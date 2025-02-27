use crate::domain::model::wife::Message;
use csv::{Reader, ReaderBuilder};
use std::error::Error;
use crate::app::app::DataDir;
use crate::app::cfg::cfg::Cfg;
use crate::domain::error::wife::WifeMessageIsNoneError;

pub trait MessageParser: Send + Sync {
    fn parse(&self) -> Result<Vec<Message>, Box<dyn Error>>;
}

pub struct CsvParser {
    cfg: Cfg
}

impl CsvParser {
    pub fn new(cfg: Cfg) -> Self {
        Self { cfg }
    }
}

impl MessageParser for CsvParser {
    fn parse(&self) -> Result<Vec<Message>, Box<dyn Error>> {
        let file = DataDir::get(self.cfg.wife_filepath.as_str()).unwrap();

        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file.data.as_ref());

        let mut messages: Vec<Message> = Vec::new();

        for (i, result) in reader.records().enumerate() {
            match result {
                Ok(record) => {
                    if let Some(str) = record.get(i) {
                        messages.push(Message::new(str.to_string()))
                    }
                },
                Err(e) => return Err(Box::new(e)),
            }
        }

        Ok(messages)
    }
}
