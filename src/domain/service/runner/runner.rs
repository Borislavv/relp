use std::thread;
use std::thread::JoinHandle;
use std::sync::{mpsc, Arc, Mutex};
use crate::infrastructure::service::message;

pub trait Runner {
    fn run(&self) -> ();
}

pub struct AppRunner {
    provider: Arc<Mutex<Box<dyn message::provider::Provider>>>,
    consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>>,
}

impl AppRunner {
    pub fn new(
        provider: Arc<Mutex<Box<dyn message::provider::Provider>>>,
        consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>>
    ) -> AppRunner {
        AppRunner { provider, consumer }
    }
}

impl Runner for AppRunner {
    fn run(&self) -> () {
        let (s, r) = mpsc::sync_channel(100);
        let provider = self.provider.clone();
        let consumer = self.consumer.clone();

        let mut threads: Vec<JoinHandle<()>> = Vec::new();

        threads.push(
            thread::spawn(move || {
                provider.lock().unwrap().provide(s);
            })
        );

        threads.push(
            thread::spawn(move || {
                consumer.lock().unwrap().consume(r);
            })
        );

        for descriptor in threads {
            descriptor.join().unwrap();
        }
    }
}