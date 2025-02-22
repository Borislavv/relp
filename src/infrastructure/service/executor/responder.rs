use crate::app::cfg::cfg::Cfg;
use crate::infrastructure::integration::telegram;
use crate::infrastructure::model::command::Exit;
use std::error::Error;
use std::sync::Arc;

pub trait Responder: Send + Sync {
    fn respond(&self, exit_state: Exit) -> Result<(), Box<dyn Error>>;
}

pub struct ExitCommandResponder {
    cfg: Cfg,
    telegram: Arc<Box<dyn telegram::facade::TelegramFacadeTrait>>,
}

impl ExitCommandResponder {
    pub fn new(
        cfg: Cfg,
        telegram: Arc<Box<dyn telegram::facade::TelegramFacadeTrait>>,
    ) -> ExitCommandResponder {
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
                match exit.input_message {
                    Some(msg) => msg.text,
                    None => "".to_string(),
                },
                exit.stdout.as_str(),
                exit.stderr.as_str(),
                exit.code,
            )
            .as_str(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
