#![allow(dead_code)]

use crate::logger::setup_logger;
use crate::main_loop::MainLoop;

mod algorithms;
mod block;
mod constants;
mod input;
mod io;
mod logger;
mod main_loop;
mod personal_world;
mod player;
mod positions;
mod renderer;
mod structures;
mod tests;
mod ui;
mod utils;
mod world;
mod world_gen;

fn main() {
    setup_logger().unwrap();

    let main_loop = MainLoop::new();
    main_loop.run();

    //let mut main_loop = MainLoop::new();
    //main_loop.run();
    //start_main_loop();
}
