use crate::app;
use std::thread;
use app::cfg::Cfg;
use crate::infrastructure;
use std::sync::{mpsc, Arc, Mutex};
use app::error::NotBootedKernelError;
use infrastructure::service::message;
use infrastructure::service::command;
use infrastructure::integration::telegram;
use crate::infrastructure::service::executor;

pub trait Kernel {
    fn run(&self) -> Result<(), NotBootedKernelError>;
}

pub trait Bootable {
    fn boot(cfg: Cfg) -> Self;
}

pub struct App {
    is_init: bool,
    provider: Arc<Mutex<Box<dyn message::provider::Provider>>>,
    consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>>,
}
impl App {
    pub fn new() -> Self {
        App::boot(Cfg::new().unwrap())
    }

    fn boot(cfg: Cfg) -> Self {
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
                Box::new(executor::executor::CommandExecutor::new(
                    Box::new(executor::responder::ExitCommandResponder::new(cfg, telegram_facade)),
                )),
                Box::new(command::factory::CommandFactory::new()),
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

        let (s, r) = mpsc::sync_channel(1);
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