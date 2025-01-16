use std::panic;
use std::sync::Arc;
use log::info;
use reqwest::Error;
use service::command;
use crate::app::cfg::Cfg;
use integration::telegram;
use crate::infrastructure::service;
use crate::infrastructure::integration;
use integration::telegram::dto::Message;

pub trait Handler: Send + panic::RefUnwindSafe {
    fn handle(&self, msg: Message) -> Result<(), Error>;
}

pub struct MessageHandler {
    cfg: Cfg,
    builder: Box<dyn command::builder::Builder>,
    processor: Box<dyn command::processor::Processor>,
    telegram: Arc<Box<dyn telegram::facade::FacadeTrait>>,
}

impl MessageHandler {
    pub fn new(
        cfg: Cfg,
        builder: Box<dyn command::builder::Builder>,
        processor: Box<dyn command::processor::Processor>,
        telegram: Arc<Box<dyn telegram::facade::FacadeTrait>>,
    ) -> MessageHandler {
        MessageHandler{ cfg, builder, processor, telegram }
    }
}

impl Handler for MessageHandler {
    fn handle(&self, msg: Message) -> Result<(), Error> {
        let binding_test = msg.text.clone();
        let binding_date = msg.text.clone();

        let text = binding_test.as_str();
        let date = binding_date.as_str();

        let cmd = self.builder.build(msg);
        let exit = self.processor.process(cmd);

        info!("[{}] Message {} was successfully handled.", date, text);

        match self.telegram.send_message(
            self.cfg.chat_id,
            format!(
                "```Input:\t{}```
                ```Stdout:\t{}```
                ```Stderr:\t{}```
                ```Code:\t{}```",
                text,
                exit.stdout.as_str(),
                exit.stderr.as_str(),
                exit.code,
            ).as_str()
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}