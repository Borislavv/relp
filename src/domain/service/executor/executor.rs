use crate::domain::model::command::Executable;
use crate::domain::model::event::ExecutableEvent;
use crate::infrastructure::service::executor::responder::Responder;
use std::error::Error;
use std::sync::Arc;

pub trait Executor: Send + Sync {
    fn exec(&self, cmd: Arc<Box<dyn ExecutableEvent>>) -> Result<(), Box<dyn Error>>;
}
pub struct CommandExecutor {
    responder: Box<dyn Responder>,
}
impl CommandExecutor {
    pub fn new(responder: Box<dyn Responder>) -> Self {
        Self { responder }
    }
}
impl Executor for CommandExecutor {
    fn exec(&self, cmd: Arc<Box<dyn ExecutableEvent>>) -> Result<(), Box<dyn Error>> {
        self.responder.respond(cmd.exec())
    }
}
