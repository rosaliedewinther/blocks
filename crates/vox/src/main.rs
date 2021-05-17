#![allow(dead_code)]
use crate::game::VoxGame;
use crate::logger::setup_logger;
use crate::positions::ChunkPos;
use crate::world::octree::{Octree, OctreeChunk, OctreeManager};
use crate::world_gen::chunk::Chunk;

mod constants;
mod game;
mod logger;
mod personal_world;
mod player;
mod positions;
mod ui;
mod utils;

fn main() {
    setup_logger().unwrap();

    let mut octree = OctreeManager::new();
    octree.increase(OctreeChunk::LeftBottomBack);
    println!("{:?}", octree);

    let mut game = VoxGame::new();
    game.run();
}
