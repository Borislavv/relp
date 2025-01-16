use log::info;
use reqwest::Error;
use crate::infrastructure::integration::telegram::dto::Message;
use crate::infrastructure::service::command::command::Executable;
use crate::infrastructure::service::command::dto::Exit;

pub trait Executor {
    fn exec(&self, cmd: Box<dyn Executable>) -> Exit;
}
pub struct CommandExecutor {

}
impl CommandExecutor {
    fn new() -> Self {
        Self{}
    }
}
impl Executor for CommandExecutor {
    fn exec(&self, msg: Message, cmd: Box<dyn Executable>) -> Result<(), Error>  {
        cmd.exec()
        let binding_test = msg.text.clone();
        let binding_date = msg.text.clone();

        let text = binding_test.as_str();
        let date = binding_date.as_str();

        let exit = ;

        info!("[{}] Message {} was successfully handled.", date, text);


    }
}