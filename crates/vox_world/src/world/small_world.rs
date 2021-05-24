use crate::blocks::block::BlockId;
use crate::player::Player;
use crate::world::world_trait::World;
use crate::world_gen::chunk::Chunk;
use crate::world_gen::meta_chunk::MetaChunk;
use rayon::prelude::ParallelSliceMut;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use vox_core::constants::{METACHUNKSIZE, METACHUNK_GEN_RANGE};
use vox_core::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};

pub struct SmallWorld {
    chunks: Vec<(MetaChunkPos, MetaChunk)>,
    pub loading_chunks: HashSet<MetaChunkPos>,
    pub world_seed: u32,
    pub time: f64,
    start_time: Instant,
}

impl SmallWorld {
    pub fn count_chunks(&self) -> i32 {
        return self.chunks.len() as i32;
    }

    pub fn get_meta_chunk(&self, pos: &MetaChunkPos) -> Option<&MetaChunk> {
        let index = self.chunks.binary_search_by(|(p, c)| p.cmp(pos));
        return match index {
            Ok(i) => {
                let maybe_chunk = self.chunks.get(i);
                match maybe_chunk {
                    None => None,
                    Some((_, chunk)) => Some(chunk),
                }
            }
            Err(_) => None,
        };
    }
    pub fn get_meta_chunk_mut(&mut self, pos: &MetaChunkPos) -> Option<&mut MetaChunk> {
        let index = self.chunks.binary_search_by(|(p, c)| p.cmp(pos));
        return match index {
            Ok(i) => {
                let maybe_chunk = self.chunks.get_mut(i);
                match maybe_chunk {
                    None => None,
                    Some((_, chunk)) => Some(chunk),
                }
            }
            Err(_) => None,
        };
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

    pub fn get_chunk_mut(&mut self, pos: &ChunkPos) -> Option<&mut Chunk> {
        if pos.y >= METACHUNKSIZE as i32 {
            return None;
        }
        let c = self.get_meta_chunk_mut(&pos.get_meta_chunk_pos());
        return match c {
            Some(chunk) => chunk.get_chunk_mut(&pos.get_local_chunk_pos()),
            None => None,
        };
    }
}
impl crate::world::world_trait::World for SmallWorld {
    fn new(seed: u32) -> SmallWorld {
        SmallWorld {
            chunks: Vec::new(),
            loading_chunks: HashSet::new(),
            world_seed: seed,
            time: 0.0,
            start_time: Instant::now(),
        }
    }
    fn set_block(&mut self, block: u8, pos: GlobalBlockPos) {
        match self.get_chunk_mut(&pos.get_chunk_pos()) {
            Some(c) => c.set_block(block, &pos.get_local_pos()),
            None => (),
        };
    }

    fn get_block(&self, pos: GlobalBlockPos) -> Option<BlockId> {
        return match self.get_chunk(&pos.get_chunk_pos()) {
            Some(c) => c.get_block(&pos.get_local_pos()),
            None => None,
        };
    }

    fn filter_chunks(&mut self, player: &Player) {
        self.chunks.retain(|(pos, _)| {
            if MetaChunk::retain_meta_chunk(player, *pos) {
                return true;
            }
            println!("remove chunk: {:?}", pos);
            return false;
        });
    }

    fn update(&mut self) {
        self.time = self.start_time.elapsed().as_secs_f64();
    }
    fn get_all_chunks(&self) -> &Vec<(MetaChunkPos, MetaChunk)> {
        return &self.chunks;
    }
    fn get_chunk(&self, pos: &ChunkPos) -> Option<&Chunk> {
        if pos.y >= METACHUNKSIZE as i32 {
            return None;
        }
        let c = self.get_meta_chunk(&pos.get_meta_chunk_pos());
        return match c {
            Some(chunk) => chunk.get_chunk(&pos.get_local_chunk_pos()),
            None => None,
        };
    }
    fn chunk_exists_or_generating(&self, pos: &MetaChunkPos) -> bool {
        if self.get_meta_chunk(pos).is_none() && !self.loading_chunks.contains(pos) {
            return false;
        }
        return true;
    }
}
