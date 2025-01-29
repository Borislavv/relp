pub mod app;
pub mod domain;
pub mod infrastructure;

use crate::app::app::Kernel;
use app::app::App;

fn main() {
    App::new().run().unwrap()
}
