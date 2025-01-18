use crate::infrastructure;
use std::sync::{Arc, Mutex};
use crate::app::cfg::cfg::Cfg;
use crate::domain::service::command;
use infrastructure::service::message;
use crate::app::model::state::{AppState, State};
use infrastructure::integration::telegram;
use crate::app::error::kernel::NotBootedKernelError;
use crate::domain::factory::command::CommandFactory;
use crate::domain::service::command::worker::Worker;
use crate::domain::service::runner::runner::{AppRunner, Runner};
use crate::domain::service::executor::executor::CommandExecutor;
use crate::infrastructure::service::execution::responder::ExitCommandResponder;

pub trait Kernel {
    fn run(&self) -> Result<(), NotBootedKernelError>;
}

pub trait Bootable {
    fn boot(cfg: Cfg) -> Self;
}

pub struct App {
    is_init: bool,
    app_runner: Box<dyn Runner>
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

        let telegram_facade: Arc<Box<dyn telegram::facade::TelegramFacadeTrait>> = Arc::new(Box::new(
            telegram::facade::TelegramFacade::new(Box::new(telegram::service::TelegramService::new(
                Box::new(telegram::http::Client::new(token, frequency))
            )))
        ));

        let state: Arc<Box<dyn State>> = Arc::new(Box::new(AppState::new()));

        let provider: Arc<Mutex<Box<dyn message::provider::Provider>>> = Arc::new(Mutex::new(Box::new(
            message::poller::LongPoller::new(frequency_cloned, state.clone(), telegram_facade.clone())
        )));

        let events_mutex = Arc::new(Mutex::new(Vec::new()));

        let consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>> = Arc::new(Mutex::new(Box::new(
            message::consumer::MessageConsumer::new(
                Box::new(CommandExecutor::new(Box::new(ExitCommandResponder::new(cfg.clone(), telegram_facade.clone())))),
                Box::new(CommandFactory::new(events_mutex.clone())),
                state.clone(),
            ),
        )));

        let worker: Arc<Box<dyn Worker>> = Arc::new(Box::new(command::worker::CommandWorker::new(
            cfg, state.clone(), events_mutex, telegram_facade)
        ));

        App{ is_init: true, app_runner: Box::new(AppRunner::new(worker, provider, consumer)) }
    }
}
impl Kernel for App {
    fn run(&self) -> Result<(), NotBootedKernelError> {
        if !self.is_init {
            Err(NotBootedKernelError::new().into())
        } else {
            Ok(self.app_runner.run())
        }
    }
}