use std::error::Error;
use crate::infrastructure::service::command::command::Executable;
use crate::infrastructure::service::executor::responder::Responder;

pub trait Executor: Send + Sync {
    fn exec(&self, cmd: Box<dyn Executable>) -> Result<(), Box<dyn Error>>;
}
pub struct CommandExecutor {
    responder: Box<dyn Responder>,
}
impl CommandExecutor {
    pub fn new(responder: Box<dyn Responder>) -> Self {
        Self{ responder }
    }
}
impl Executor for CommandExecutor {
    fn exec(&self, cmd: Box<dyn Executable>) -> Result<(), Box<dyn Error>>  {
        self.responder.respond(cmd.exec())
    }
}