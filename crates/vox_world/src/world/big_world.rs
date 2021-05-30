use crate::blocks::block::{get_blockid, BlockId};
use crate::blocks::block_type::BlockType;
use crate::player::Player;
use crate::world_gen::chunk::Chunk;
use crate::world_gen::meta_chunk::MetaChunk;
use nalgebra::Vector3;
use rayon::prelude::ParallelSliceMut;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use vox_core::constants::{METACHUNKSIZE, METACHUNK_GEN_RANGE};
use vox_core::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};

pub struct BigWorld {
    //meta_chunk_locations: [[i32; 3]; 27], //one brickmap which the playes is currently in and all around the player
    brickmap: Box<[u32; 64]>, //assumes brickmaps with size 4^3
    bricks: Vec<[u8; 512]>,   //brick with size 8^3
    pub loading_chunks: HashSet<MetaChunkPos>,
    pub world_seed: u32,
    pub time: f64,
    start_time: Instant,
}

impl BigWorld {
    #[inline]
    pub fn get_block(&self, pos: GlobalBlockPos) -> Option<BlockId> {
        return None;
    }
    pub fn new(seed: u32) -> BigWorld {
        let mut brickmap = Box::new([u32::MAX; 64]);
        let mut bricks = vec![];
        for i in 0..brickmap.len() {
            brickmap[i] = i as u32;
            let mut temp_brick = [0u8; 512];
            temp_brick[i] = get_blockid(BlockType::Grass);
            bricks.push(temp_brick)
        }
        BigWorld {
            brickmap,
            bricks,
            loading_chunks: HashSet::new(),
            world_seed: seed,
            time: 0.0,
            start_time: Instant::now(),
        }
    }
    pub fn set_block(&mut self, block: u8, pos: GlobalBlockPos) {
        todo!()
    }

    pub fn filter_chunks(&mut self, player: &Player) {
        todo!()
    }

    pub fn update(&mut self) {
        self.time = self.start_time.elapsed().as_secs_f64();
    }
    pub fn chunk_exists_or_generating(&self, pos: &MetaChunkPos) -> bool {
        todo!()
    }
}