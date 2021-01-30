use crate::block::{Block, BlockSides};
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};
use crate::world_gen::chunk::Chunk;
use crate::world_gen::chunk_loader::ChunkLoader;
use crate::world_gen::meta_chunk::MetaChunk;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct World {
    pub chunks: HashMap<MetaChunkPos, MetaChunk>,
    pub loading_chunks: HashSet<MetaChunkPos>,
    pub world_seed: u32,
    pub chunk_loader: ChunkLoader,
    pub players: HashMap<String, Player>,
    pub time: f64,
    start_time: Instant,
}

impl World {
    pub fn new(seed: u32) -> World {
        World {
            chunks: HashMap::new(),
            loading_chunks: HashSet::new(),
            world_seed: seed,
            chunk_loader: ChunkLoader::new(),
            players: HashMap::new(),
            time: 0.0,
            start_time: Instant::now(),
        }
    }
    pub fn update(&mut self) {
        self.time = self.start_time.elapsed().as_secs_f64();
    }
    pub fn add_player(&mut self, name: String, player: Player) {
        self.players.insert(name, player);
    }
    pub fn get_block(&self, pos: &GlobalBlockPos) -> Option<&Block> {
        return match self.chunks.get(&pos.get_meta_chunk_pos()) {
            Some(c) => c.get_block(pos),
            None => None,
        };
    }
    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        let c = self.chunks.get(&pos.get_meta_chunk_pos());
        return match c {
            Some(chunk) => chunk.get_chunk(&pos.get_local_chunk_pos()),
            None => None,
        };
    }
    pub fn chunk_exists_or_generating(&self, pos: &MetaChunkPos) -> bool {
        if self.chunks.get(pos).is_none() && !self.loading_chunks.contains(pos) {
            return false;
        }
        return true;
    }
}
