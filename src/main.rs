#![allow(dead_code)]

use crate::logger::setup_logger;
use crate::main_loop::MainLoop;
use crate::world::octree::{Octree, OctreeChunk, OctreeManager};

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

    let mut octree = OctreeManager::new();
    octree.increase(OctreeChunk::LeftBottomBack);
    println!("{:?}", octree);

    let main_loop = MainLoop::new();
    main_loop.run();
}
