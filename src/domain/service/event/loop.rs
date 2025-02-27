use crate::app;
use crate::domain::model::event::ExecutableEvent;
use crate::domain::r#enum::event::Repeat;
use crate::domain::service::executor::executor::Executor;
use crate::infrastructure::broadcasting::mpsc::channel::Channel;
use std::sync::{Arc, Mutex};

pub trait EventLoop: Send + Sync {
    fn serve(&self);
    fn add_event(&self, event: Arc<Box<dyn ExecutableEvent>>);
}

pub struct CommandEventLoop {
    state: Arc<Box<dyn app::model::state::State>>,
    events: Arc<Mutex<Vec<Arc<Box<dyn ExecutableEvent>>>>>,
    channel: Box<dyn Channel<Arc<Box<dyn ExecutableEvent>>>>,
    executor: Arc<Box<dyn Executor>>,
}

impl CommandEventLoop {
    pub fn new(
        state: Arc<Box<dyn app::model::state::State>>,
        channel: Box<dyn Channel<Arc<Box<dyn ExecutableEvent>>>>,
        executor: Arc<Box<dyn Executor>>,
    ) -> Self {
        Self { state, events: Arc::new(Mutex::new(vec![])), channel, executor }
    }
}

impl CommandEventLoop {
    fn handle_event_repeats(&self, event: Arc<Box<dyn ExecutableEvent>>) {
        match event.repeats() {
            Repeat::Once => {}
            Repeat::Always => {
                // return an event back always
                // TODO potentially memory licking (arc or mutex can handle links permanently)
                self.events.lock().unwrap().push(event.clone());
            }
            Repeat::Times(n) => {
                if n > 0 {
                    // return an event back N times
                    // TODO potentially memory licking (arc or mutex can handle links permanently)
                    self.events.lock().unwrap().push(event.clone());
                }
            }
        }
    }
}

impl EventLoop for CommandEventLoop {
    // Method serve is infinitely iterating over events and checks whether one or
    // more events will be ready for send to them Receivers or execute at place
    // (depends on if an event has a sender into or not). Technical details:
    // have a timeout between each new iteration in 1 second due to decrease CPU
    // consumption.
    fn serve(&self) {
        loop {
            // check the application status is still active
            if self.state.is_closed() {
                return;
            }

            // (vec![]).drain(0..) -> removes all elements from vector, if an event is not ready, you need put it back
            for event in self.events.lock().unwrap().drain(0..) {
                if event.is_ready() {
                    match event.sender() {
                        Some(ready_event) => {
                            // unwrap is safe if you do not have a failures in another thread which consume from receiver
                            ready_event.send(event.clone()).unwrap();
                            // back event to the heap if necessary
                            self.handle_event_repeats(event);
                        }
                        None => {
                            match self.executor.exec(event.clone()) {
                                Ok(_) => {
                                    let name = event.name().to_string();
                                    // back event to the heap if necessary
                                    self.handle_event_repeats(event);
                                    println!("Command: {} successfully executed.", name)
                                }
                                Err(e) => {
                                    let name = event.name().to_string();
                                    self.events.lock().unwrap().push(event);
                                    println!("Error: {} occurred while execution command: {}.", e, name)
                                }
                            };
                        }
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

    // add_event - pushes new event into the loop event heap and creates a new one
    // pair of Sender and Receiver for event.key() if necessary of course.
    fn add_event(&self, event: Arc<Box<dyn ExecutableEvent>>) {
        // handle lock and push new event into the loop
        self.events.lock().unwrap().push(event);
    }
}
