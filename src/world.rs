use crate::block::{Block, BlockSides};
use crate::player::Player;
use crate::positions::{GlobalBlockPos, MetaChunkPos};
use crate::world_gen::chunk_loader::ChunkLoader;
use crate::world_gen::meta_chunk::MetaChunk;
use std::collections::{HashMap, HashSet};

pub struct World {
    pub chunks: HashMap<MetaChunkPos, MetaChunk>,
    pub loading_chunks: HashSet<MetaChunkPos>,
    pub world_seed: u32,
    pub chunk_loader: ChunkLoader,
    pub players: HashMap<String, Player>,
}

impl World {
    pub fn new(seed: u32) -> World {
        World {
            chunks: HashMap::new(),
            loading_chunks: HashSet::new(),
            world_seed: seed,
            chunk_loader: ChunkLoader::new(),
            players: HashMap::new(),
        }
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
    pub fn chunk_exists_or_generating(&self, pos: &MetaChunkPos) -> bool {
        if self.chunks.get(pos).is_none() && !self.loading_chunks.contains(pos) {
            return false;
        }
        return true;
    }

    pub fn sides_to_render(&self, global_pos: &GlobalBlockPos) -> BlockSides {
        let mut sides = BlockSides::new();
        if self.should_render_against_block(&global_pos.get_diff(1, 0, 0)) {
            sides.right = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(-1, 0, 0)) {
            sides.left = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, 1, 0)) {
            sides.top = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, -1, 0)) {
            sides.bot = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, 0, 1)) {
            sides.back = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, 0, -1)) {
            sides.front = true;
        }
        return sides;
    }
    pub fn should_render_against_block(&self, pos: &GlobalBlockPos) -> bool {
        let real_chunk_pos = pos.get_meta_chunk_pos();
        if !self.chunks.contains_key(&real_chunk_pos) {
            return false;
        }
        let block = self.get_block(&pos);
        match block {
            Some(b) => b.should_render_against(),
            None => true,
        }
    }
}
