use crate::app;
use crate::domain::service::event::model::{Event, ExecutableEvent};
use crate::domain::service::executor::executor::Executor;
use crate::infrastructure::broadcasting::mpsc::channel::Channel;
use std::sync::{Arc, Mutex};

pub trait EventLoop: Send + Sync {
    fn serve(&self);
    fn add_event(&self, event: Arc<Box<dyn Event>>);
}

pub struct CommandEventLoop {
    state: Arc<Box<dyn app::model::state::State>>,
    events: Mutex<Vec<Arc<Box<dyn ExecutableEvent>>>>,
    channel: Box<dyn Channel<Arc<Box<dyn ExecutableEvent>>>>,
    executor: Arc<Box<dyn Executor>>,
}

impl CommandEventLoop {
    pub fn new(
        state: Arc<Box<dyn app::model::state::State>>,
        channel: Box<dyn Channel<Arc<Box<dyn Event>>>>,
        executor: Arc<Box<dyn Executor>>
    ) -> Self {
        Self { state, events: Mutex::new(vec![]), channel, executor }
    }
}

impl EventLoop for CommandEventLoop {
    // Method serve is infinitely iterating over events and checks whether one or more events
    // will be ready for send to them Receivers or execute at place (depends on if an event has a sender into or not).
    // Technical details: have a timeout between each new iteration in 1 second due to decrease CPU consumption.
    fn serve(&self) {
        loop {
            // check the application status is still active
            if self.state.is_closed() { return; }

            // (vec![]).drain(0..) -> removes all elements from vector, if an event is not ready, you need put it back
            for event in self.events.lock().unwrap().drain(0..) {
                if event.is_ready() {
                    match event.sender() {
                        Some(readyEvent) => {
                            // unwrap is safe if you do not have a failures in another thread which consume from receiver
                            readyEvent.send(event).unwrap()
                        },
                        None => {
                            match self.executor.exec(event.clone()) {
                                Ok(_) => println!("Command: {} successfully executed.", event.name()),
                                Err(e) => println!("Error: {} occurred while execution command: {}.", e, event.name()),
                            };
                        },
                    };
                } else {
                    // return the event which is not ready back to the vector
                    self.events.lock().unwrap().push(event);
                }
            }
            // limitation of CPU consumption (once in a second)
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    // add_event - pushes new event into the loop event heap and creates a new one pair
    // of Sender and Receiver for event.key() if necessary of course.
    fn add_event(&self, event: Arc<Box<dyn ExecutableEvent>>) {
        // handle lock and push new event into the loop
        self.events.lock().unwrap().push(event);
    }
}