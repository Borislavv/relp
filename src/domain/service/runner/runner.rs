use std::thread;
use std::thread::JoinHandle;
use std::sync::{mpsc, Arc, Mutex};
use crate::domain::service::command;
use crate::infrastructure::service::message;

pub trait Runner {
    fn run(&self) -> ();
}

pub struct AppRunner {
    worker: Arc<Box<dyn command::worker::Worker>>,
    provider: Arc<Mutex<Box<dyn message::provider::Provider>>>,
    consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>>,
}

impl AppRunner {
    pub fn new(
        worker: Arc<Box<dyn command::worker::Worker>>,
        provider: Arc<Mutex<Box<dyn message::provider::Provider>>>,
        consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>>,
    ) -> AppRunner {
        AppRunner { worker, provider, consumer }
    }
}

impl Runner for AppRunner {
    fn run(&self) -> () {
        let (s, r) = mpsc::sync_channel(100);
        let worker = self.worker.clone();
        let provider = self.provider.clone();
        let consumer = self.consumer.clone();

        let mut threads: Vec<JoinHandle<()>> = Vec::new();

        threads.push(
            thread::spawn(move || {
                worker.serve();
            })
        );

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