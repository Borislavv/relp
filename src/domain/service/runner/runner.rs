use crate::app::cfg::cfg::Cfg;
use crate::domain::service::command;
use crate::domain::service::event::r#loop::EventLoop;
use crate::infrastructure::service::message;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub trait Runner {
    fn run(&self) -> ();
}

pub struct AppRunner {
    cfg: Cfg,
    event_loop: Arc<Box<dyn EventLoop>>,
    worker: Arc<Box<dyn command::worker::Worker>>,
    provider: Arc<Mutex<Box<dyn message::provider::Provider>>>,
    consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>>,
}

impl AppRunner {
    pub fn new(
        cfg: Cfg,
        event_loop: Arc<Box<dyn EventLoop>>,
        worker: Arc<Box<dyn command::worker::Worker>>,
        provider: Arc<Mutex<Box<dyn message::provider::Provider>>>,
        consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>>,
    ) -> AppRunner {
        AppRunner { cfg, event_loop, worker, provider, consumer }
    }
}

impl Runner for AppRunner {
    fn run(&self) -> () {
        let (sender, receiver) = mpsc::sync_channel(self.cfg.event_loop_channel_capacity);
        let worker = self.worker.clone();
        let provider = self.provider.clone();
        let consumer = self.consumer.clone();
        let event_loop = self.event_loop.clone();

        let mut threads: Vec<JoinHandle<()>> = Vec::new();

        threads.push(thread::spawn(move || {
            // starts a daemon for serve deferred events.
            worker.serve();
        }));

        threads.push(thread::spawn(move || {
            // starts a long poller which asks telegram message updates and provides them into output channel.
            provider.lock().unwrap().provide(sender);
        }));

        threads.push(thread::spawn(move || {
            // starts a consumer of channel which uses provider as the output.
            consumer.lock().unwrap().consume(receiver);
        }));

        threads.push(thread::spawn(move || {
            // starts an event loop which will serve different events in app
            event_loop.serve();
        }));

        for descriptor in threads {
            descriptor.join().unwrap();
        }
    }
}