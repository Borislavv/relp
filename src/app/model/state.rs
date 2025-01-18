use std::sync::atomic::{AtomicBool, Ordering};

pub trait State: Send + Sync {
    fn close(&self);
    fn is_closed(&self) -> bool;
}

pub struct AppState {
    c: AtomicBool,
}
impl AppState {
    pub fn new() -> AppState {
        AppState { c: AtomicBool::new(false) }
    }
}
impl State for AppState {
    fn close(&self) {
        for _ in 0..1000 {
            match self.c.compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => return,
                Err(_) => panic!("app::model::state: unable to close application (CAS challenge has been failed)"),
            }
        }
    }
    fn is_closed(&self) -> bool {
        self.c.load(Ordering::SeqCst)
    }
}