use crate::block::Block;
use crate::constants::CHUNKSIZE;
use crate::positions::{ChunkPos, LocalBlockPos};
use crate::world_gen::basic::ChunkGenerator;
use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    blocks: Vec<Block>,
}

impl Chunk {
    pub fn new(blocks: Vec<Block>) -> Chunk {
        Chunk { blocks }
    }
    pub fn generate(pos: &ChunkPos, seed: u32) -> Chunk {
        let chunk_generator = ChunkGenerator::new(seed);
        return chunk_generator.full_generation_pass(pos);
    }

    pub fn update(&mut self, _dt: f32) -> bool {
        return false;
    }

    pub fn set_block(&mut self, block: Block, pos: &LocalBlockPos) {
        if pos.x < 0
            || pos.x > (CHUNKSIZE - 1) as i32
            || pos.y < 0
            || pos.y > (CHUNKSIZE - 1) as i32
            || pos.z < 0
            || pos.z > (CHUNKSIZE - 1) as i32
        {
            warn!("tried to place block outside chunk with pos: {:?}", &pos);
            return;
        }
        self.blocks[pos.x as usize
            + pos.y as usize * CHUNKSIZE as usize
            + pos.z as usize * CHUNKSIZE as usize * CHUNKSIZE as usize] = block;
    }
    pub fn get_block(&self, pos: &LocalBlockPos) -> Option<&Block> {
        if pos.x < 0
            || pos.x >= (CHUNKSIZE) as i32
            || pos.y < 0
            || pos.y >= (CHUNKSIZE) as i32
            || pos.z < 0
            || pos.z >= (CHUNKSIZE) as i32
        {
            //println!("couldn't get block at: {:?}", &pos);
            return None;
        }
        return Some(
            &self.blocks[pos.x as usize
                + pos.y as usize * CHUNKSIZE as usize
                + pos.z as usize * CHUNKSIZE as usize * CHUNKSIZE as usize],
        );
    }
}
