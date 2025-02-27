use crate::app::cfg::cfg::Cfg;
use crate::app::error::kernel::NotBootedKernelError;
use crate::app::model::state::{AppState, State};
use crate::domain::factory::command::CommandFactory;
use crate::domain::model::command::WifeMessageCmd;
use crate::domain::model::event::ExecutableEvent;
use crate::domain::service::event::r#loop::{CommandEventLoop, EventLoop};
use crate::domain::service::executor::executor::{CommandExecutor, Executor};
use crate::domain::service::runner::runner::{AppRunner, Runner};
use crate::domain::service::wife::message::parser::CsvParser;
use crate::domain::service::wife::message::service::{MessageService, MessageServiceTrait};
use crate::infrastructure;
use crate::infrastructure::broadcasting::mpsc::channel::{Chan, Channel};
use crate::infrastructure::service::executor::responder::ExitCommandResponder;
use crate::infrastructure::service::message::poller::LongPoller;
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use infrastructure::integration::telegram;
use infrastructure::service::message;
use std::sync::{Arc, Mutex};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/app/data"]
pub struct DataDir;

pub trait Kernel {
    fn run(&self) -> Result<(), NotBootedKernelError>;
}

pub trait Bootable {
    fn boot(cfg: Cfg) -> Self;
}

pub struct App {
    is_init: bool,
    app_runner: Box<dyn Runner>,
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

        let telegram_facade: Arc<Box<dyn telegram::facade::TelegramFacadeTrait>> =
            Arc::new(Box::new(telegram::facade::TelegramFacade::new(Box::new(
                telegram::service::TelegramService::new(Box::new(telegram::http::Client::new(
                    token, frequency,
                ))),
            ))));

        let executor: Arc<Box<dyn Executor>> = Arc::new(Box::new(CommandExecutor::new(Box::new(
            ExitCommandResponder::new(cfg.clone(), telegram_facade.clone()),
        ))));

        let chan: Chan<Arc<Box<dyn ExecutableEvent>>> = Chan::new();
        let state: Arc<Box<dyn State>> = Arc::new(Box::new(AppState::new()));
        let channel: Box<dyn Channel<Arc<Box<dyn ExecutableEvent>>>> = Box::new(chan);
        let event_loop: Arc<Box<dyn EventLoop>> = Arc::new(Box::new(CommandEventLoop::new(
            state.clone(),
            channel,
            executor.clone(),
        )));

        let now = Local::now().naive_local();
        let date = NaiveDateTime::from(NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap().and_hms_opt(10, 0, 0).unwrap());
        let message_service: Arc<Box<dyn MessageServiceTrait>> = Arc::new(Box::new(MessageService::new(
            Arc::new(Box::new(CsvParser::new(cfg.clone())
        ))).unwrap()));
        event_loop.add_event(Arc::new(Box::new(WifeMessageCmd::new(
            date.clone(), "С добрым утром моя радость, я безумно тебя люблю.".to_string(), message_service.clone(),
        ))));
        event_loop.add_event(Arc::new(Box::new(WifeMessageCmd::new(
            date, "Спокойной ночи, моя любовь, сладких снов.".to_string(), message_service,
        ))));

        let provider: Arc<Mutex<Box<dyn message::provider::Provider>>> =
            Arc::new(Mutex::new(Box::new(LongPoller::new(
                cfg.clone(),
                frequency_cloned,
                state.clone(),
                telegram_facade.clone(),
            ))));

        let events_mutex = Arc::new(Mutex::new(Vec::new()));

        let consumer: Arc<Mutex<Box<dyn message::consumer::Consumer>>> = Arc::new(Mutex::new(
            Box::new(message::consumer::MessageConsumer::new(
                Box::new(CommandFactory::new(
                    events_mutex.clone(),
                    event_loop.clone(),
                )),
                state.clone(),
                event_loop.clone(),
            )),
        ));

        App {
            is_init: true,
            app_runner: Box::new(AppRunner::new(cfg, event_loop, provider, consumer)),
        }
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
