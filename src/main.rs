pub mod app;
pub mod domain;
pub mod infrastructure;

use app::app::App;
use crate::app::app::Kernel;

fn main() {
    App::new().run().unwrap()
}
