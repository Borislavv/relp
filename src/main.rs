#![feature(duration_constructors)]

pub mod app;
pub mod domain;
pub mod infrastructure;

use crate::app::app::Kernel;
use app::app::App;
use std::ops::Add;

fn main() {
    App::new().run().unwrap()
}