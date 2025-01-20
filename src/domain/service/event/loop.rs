use crate::app;
use std::sync::Mutex;
use crate::domain::service::event::model::Event;
use crate::infrastructure::broadcasting::mpsc::channel::Channel;

pub trait EventLoop {
    fn serve(&self);
    fn add_event(&self, event: Box<dyn Event>);
}

pub struct CommandEventLoop {
    heap: Mutex<Vec<Box<dyn Event>>>,
    state: Box<dyn app::model::state::State>,
    channel: Box<dyn Channel<Box<dyn Event>>>,
}

impl CommandEventLoop {
    pub fn new(state: Box<dyn app::model::state::State>, channel: Box<dyn Channel<Box<dyn Event>>>) -> Self {
        Self { heap: Mutex::new(vec![]), state, channel }
    }
}

impl EventLoop for CommandEventLoop {
    // serve - infinitely iterating over events and checks whether one or more events
    // will be ready for send to them Receivers.
    //
    // Technical details: have a timeout between each new iteration in 1 second due to decrease CPU consumption.
    fn serve(&self) {
        loop {
            // check the application status is still active
            if self.state.is_closed() { return; }

            // (vec![]).drain(0..) -> removes all elements from vector, if an event is not ready, you need put it back
            for event in self.heap.lock().unwrap().drain(0..) {
                let key = event.key();

                if event.is_ready() {
                    if let Err(e) = self.channel.sender(key.clone()) {
                        eprintln!("Sender was not found into the map, the Event was skipped: {}.", e);

                        // return the event which hasn't an own sender back to the heap
                        self.heap.lock().unwrap().push(event);

                        // create missed Sender and Receiver of current Event
                        self.channel.add(key);

                        // go to new iteration
                        continue;
                    }

                    if let Some(sender) = self.channel.sender(key.clone()) {
                        // send the ready event to the receiver (already removed from the events vector)
                        sender.send(event).unwrap();
                    }
                } else {
                    // return the event which is not ready back to the vector
                    self.heap.lock().unwrap().push(event);
                }
            }
            // limitation of CPU consumption (once in a second)
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    // add_event - pushes new event into the loop event heap and creates a new one pair
    // of Sender and Receiver for event.key() if necessary of course.
    fn add_event(&self, event: Box<dyn Event>) {
        // if (sender, receiver) are not exist for event.key()
        if let Err(_) = self.channel.sender(event.key()) {
            // then add new (sender, receiver) by key
            self.channel.add(event.key());
        }
        // handle lock and push new event into the loop
        self.heap.lock().unwrap().push(event);
    }
}