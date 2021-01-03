#![allow(dead_code)]

use log::info;

use crate::logger::setup_logger;
use crate::main_loop::MainLoop;
use crate::renderer::wgpu::start_main_loop;

mod algorithms;
mod block;
mod chunk_manager;
mod constants;
mod input;
mod io;
mod logger;
mod main_loop;
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

    info!("starting up");

    //let mut main_loop = MainLoop::new();
    //main_loop.run();
    start_main_loop();

    println!("done");
}
