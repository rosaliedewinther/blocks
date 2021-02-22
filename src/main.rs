#![allow(dead_code)]

use crate::game::VoxGame;
use crate::logger::setup_logger;
use crate::world::octree::{Octree, OctreeChunk, OctreeManager};

mod algorithms;
mod block;
mod constants;
mod game;
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

    let mut game = VoxGame::new();
    game.run();
}
