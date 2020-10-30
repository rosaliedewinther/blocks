use crate::chunk_manager::ChunkManager;
use crate::constants::METACHUNK_GEN_RANGE;
use crate::player::Player;
use crate::positions::{ChunkPos, MetaChunkPos};
use crate::world::World;
use glium::{Vertex, VertexBuffer};
use std::collections::HashMap;

pub struct ChunkLoader {}

impl ChunkLoader {
    pub fn new() -> ChunkLoader {
        ChunkLoader {}
    }
}
