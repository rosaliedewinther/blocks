#[macro_use]
extern crate glium;

use log::info;

use crate::logger::setup_logger;
use crate::main_loop::MainLoop;

mod block;
mod chunk;
mod chunk_manager;
mod constants;
mod input;
mod logger;
mod main_loop;
mod player;
mod positions;
mod renderer;
mod ui;
mod utils;
mod world;
mod world_gen;

fn main() {
    setup_logger().unwrap();

    info!("starting up");

    let mut main_loop = MainLoop::new();
    main_loop.run();

    println!("done dumping");
}
