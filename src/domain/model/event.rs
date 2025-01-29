use crate::domain::model::command::Executable;
use crate::domain::r#enum::event::Repeat;
use std::any::Any;
use std::sync::mpsc::Sender;
use std::sync::Arc;

pub trait ExecutableEvent: Executable + Event + Send + Sync {
    // if the sender is None, then the event loop will execute a cmd himself,
    // otherwise will send an event through sender.
    fn sender(&self) -> Option<Arc<Sender<Arc<Box<dyn ExecutableEvent>>>>>;
}

pub trait Event: Send + Sync + Any {
    // name returns a name of event
    fn name(&self) -> String;
    // is_ready automatically checks the current event is ready for sending
    fn is_ready(&self) -> bool;
    // repeats tells the event loop how many times an event must be executed
    fn repeats(&self) -> Repeat;
}
