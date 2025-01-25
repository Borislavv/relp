use crate::app::cfg::cfg::Cfg;
use crate::app::error::kernel::NotBootedKernelError;
use crate::app::model::state::{AppState, State};
use crate::domain::factory::command::CommandFactory;
use crate::domain::service::command;
use crate::domain::service::command::worker::Worker;
use crate::domain::service::event::model::Event;
use crate::domain::service::event::r#loop::{CommandEventLoop, EventLoop};
use crate::domain::service::executor::executor::CommandExecutor;
use crate::domain::service::runner::runner::{AppRunner, Runner};
use crate::infrastructure;
use crate::infrastructure::broadcasting::mpsc::channel::{Chan, Channel};
use crate::infrastructure::service::executor::responder::ExitCommandResponder;
use infrastructure::integration::telegram;
use infrastructure::service::message;
use std::sync::{Arc, Mutex};

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
        let channel: Box<dyn Channel<Arc<Box<dyn Event>>>> = Box::new(Chan::new());
        let event_loop: Arc<Box<dyn EventLoop>> = Arc::new(Box::new(CommandEventLoop::new(state.clone(), channel)));

        let provider: Arc<Mutex<Box<dyn message::provider::Provider>>> = Arc::new(Mutex::new(Box::new(
            message::provider::LongPoller::new(cfg.clone(), frequency_cloned, state.clone(), event_loop.clone(), telegram_facade.clone())
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
            cfg.clone(), state, events_mutex, telegram_facade)
        ));


        App{ is_init: true, app_runner: Box::new(AppRunner::new(cfg, event_loop, worker, provider, consumer)) }
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