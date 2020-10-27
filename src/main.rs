#[macro_use]
extern crate glium;

use log::info;

use crate::logger::setup_logger;
use crate::main_loop::MainLoop;
use crate::positions::MetaChunkPos;
use crate::world_gen::meta_chunk::MetaChunk;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
mod ui;
mod utils;
mod world;
mod world_gen;

fn main() {
    setup_logger().unwrap();

    info!("starting up");

    let started = Instant::now();
    let c = MetaChunk::load_or_gen(MetaChunkPos { x: 0, y: 0, z: 0 }, 0);
    println!("load: {}", started.elapsed().as_secs_f32());
    sleep(Duration::new(10, 0));
    let started = Instant::now();
    c.save_to_disk();
    println!("save: {}", started.elapsed().as_secs_f32());

    return;

    //let mut main_loop = MainLoop::new();
    //main_loop.run();

    //println!("done dumping");
}
