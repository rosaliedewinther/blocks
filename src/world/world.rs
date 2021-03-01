use crate::blocks::block::BlockId;
use crate::constants::{METACHUNKSIZE, METACHUNK_GEN_RANGE};
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};
use crate::world_gen::chunk::Chunk;
use crate::world_gen::chunk_loader::ChunkLoader;
use crate::world_gen::meta_chunk::MetaChunk;
use rayon::prelude::ParallelSliceMut;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct World {
    chunks: Vec<(MetaChunkPos, MetaChunk)>,
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
            chunks: Vec::new(),
            loading_chunks: HashSet::new(),
            world_seed: seed,
            chunk_loader: ChunkLoader::new(),
            players: HashMap::new(),
            time: 0.0,
            start_time: Instant::now(),
        }
    }
    pub fn count_chunks(&self) -> i32 {
        return self.chunks.len() as i32;
    }
    pub fn update(&mut self) {
        self.time = self.start_time.elapsed().as_secs_f64();
    }
    pub fn add_player(&mut self, name: String, player: Player) {
        self.players.insert(name, player);
    }
    pub fn get_meta_chunk(&self, pos: &MetaChunkPos) -> Option<&MetaChunk> {
        let index = self.chunks.binary_search_by(|(p, c)| p.cmp(pos));
        match index {
            Ok(i) => {
                let maybe_chunk = self.chunks.get(i);
                match maybe_chunk {
                    None => return None,
                    Some((_, chunk)) => return Some(chunk),
                }
            }
            Err(_) => return None,
        };
    }
    pub fn filter_chunks(&mut self) {
        let current_chunk = self.player.position.get_meta_chunk();
        self.chunks.retain(|(pos, _)|{
            if pos.x > current_chunk.x - METACHUNK_GEN_RANGE as i32 - 2{

            }
        });
        for (pos, _) in  {}

        for x in current_chunk.x - METACHUNK_GEN_RANGE as i32 - 2
            ..current_chunk.x + METACHUNK_GEN_RANGE as i32 + 2
        {
            for z in current_chunk.z - METACHUNK_GEN_RANGE as i32 - 2
                ..current_chunk.z + METACHUNK_GEN_RANGE as i32 + 2
            {}
        }
    }
    pub fn get_all_chunks(&self) -> &Vec<(MetaChunkPos, MetaChunk)> {
        return &self.chunks;
    }
    pub fn add_chunk(&mut self, pos: MetaChunkPos, chunk: MetaChunk) {
        self.chunks.push((pos, chunk));
        self.chunks
            .par_sort_unstable_by(|(p1, _), (p2, _)| p1.cmp(p2))
    }
    #[inline]
    pub fn get_block(&self, pos: &GlobalBlockPos) -> Option<BlockId> {
        return match self.get_chunk(&pos.get_chunk_pos()) {
            Some(c) => c.get_block(&pos.get_local_pos()),
            None => None,
        };
    }
    #[inline]
    pub fn get_block_unsafe(&self, pos: &GlobalBlockPos) -> BlockId {
        let chunk = self.get_chunk_unsafe(&pos.get_chunk_pos());
        chunk.get_block_unsafe(&pos.get_local_pos())
    }
    pub fn get_chunk_unsafe(&self, pos: &ChunkPos) -> &Chunk {
        let meta_chunk = self.get_meta_chunk(&pos.get_meta_chunk_pos()).unwrap();
        meta_chunk.get_chunk_unsafe(&pos.get_local_chunk_pos())
    }
    pub fn get_chunk(&self, pos: &ChunkPos) -> Option<&Chunk> {
        if pos.y >= METACHUNKSIZE as i32 {
            return None;
        }
        let c = self.get_meta_chunk(&pos.get_meta_chunk_pos());
        return match c {
            Some(chunk) => chunk.get_chunk(&pos.get_local_chunk_pos()),
            None => None,
        };
    }
    pub fn chunk_exists_or_generating(&self, pos: &MetaChunkPos) -> bool {
        if self.get_meta_chunk(pos).is_none() && !self.loading_chunks.contains(pos) {
            return false;
        }
        return true;
    }
}
