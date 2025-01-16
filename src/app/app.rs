use crate::app;
use std::thread;
use app::cfg::Cfg;
use crate::infrastructure;
use std::sync::{mpsc, Arc, Mutex};
use app::error::NotBootedKernelError;
use infrastructure::service::message;
use infrastructure::service::command;
use infrastructure::integration::telegram;

pub trait Kernel {
    fn run(&self) -> Result<(), NotBootedKernelError>;
}

pub struct App {
    is_init: bool,
    provider: Arc<Mutex<Box<dyn message::provider::Provider>>>,
    consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>>,
}
impl App {
    pub fn new() -> Self {
        App::boot(&Cfg::new().unwrap())
    }

    fn boot(cfg: &Cfg) -> Self {
        env_logger::init();

        let token = cfg.token.clone();
        let frequency = cfg.poll_frequency.clone();
        let frequency_cloned = frequency.clone();

        let telegram_facade: Arc<Box<dyn telegram::facade::FacadeTrait>> = Arc::new(Box::new(
            telegram::facade::Facade::new(
                Box::new(telegram::service::Service::new(
                    Box::new(telegram::http::Client::new(token, frequency))
                ))
            )
        ));

        let provider: Box<dyn message::provider::Provider> = Box::new(
            message::poller::LongPoller::new(frequency_cloned, telegram_facade.clone())
        );

        let consumer: Box<dyn message::consumer::Consumer> = Box::new(
            message::consumer::MessageConsumer::new(
                Box::new(message::handler::MessageHandler::new(
                    cfg.clone(),
                    Box::new(command::builder::CommandBuilder::new()),
                    Box::new(command::processor::CommandProcessor::new()),
                    telegram_facade,
                ))
            )
        );

         App{
             is_init: true,
             provider: Arc::new(Mutex::new(provider)),
             consumer: Arc::new(Mutex::new(consumer)),
        }
    }
}
impl Kernel for App {
    fn run(&self) -> Result<(), NotBootedKernelError> {
        if !self.is_init {
            return Err(NotBootedKernelError::new().into());
        }

        let (s, r) = mpsc::channel();
        let provider = self.provider.clone();
        let consumer = self.consumer.clone();

        let threads = vec![
            thread::spawn(move || {
                provider.lock().unwrap().provide(s);
            }),
            thread::spawn(move || {
                consumer.lock().unwrap().consume(r);
            })
        ];

        for descriptor in threads {
            descriptor.join().unwrap();
        }

        Ok(())
    }
}