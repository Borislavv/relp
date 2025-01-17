use std::sync::Arc;
use std::error::Error;
use crate::app::cfg::cfg::Cfg;
use crate::infrastructure::model::command::Exit;
use crate::infrastructure::integration::telegram;

pub trait Responder: Send + Sync {
    fn respond(&self, exit_state: Exit) -> Result<(), Box<dyn Error>>;
}

pub struct ExitCommandResponder {
    cfg: Cfg,
    telegram: Arc<Box<dyn telegram::facade::FacadeTrait>>,
}

impl ExitCommandResponder {
    pub fn new(cfg: Cfg, telegram: Arc<Box<dyn telegram::facade::FacadeTrait>>) -> ExitCommandResponder {
        ExitCommandResponder { cfg, telegram }
    }
}

impl Responder for ExitCommandResponder {
    fn respond(&self, exit: Exit) -> Result<(), Box<dyn Error>> {
        match self.telegram.send_message(
            self.cfg.chat_id,
            format!(
                "```Input:\t{}```
                ```Stdout:\t{}```
                ```Stderr:\t{}```
                ```Code:\t{}```",
                exit.message.unwrap().text,
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