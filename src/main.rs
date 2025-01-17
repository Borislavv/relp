pub mod app;
pub mod infrastructure;
mod domain;

use app::app::App;
use crate::app::app::Kernel;

fn main() {
    App::new().run().unwrap()
}
