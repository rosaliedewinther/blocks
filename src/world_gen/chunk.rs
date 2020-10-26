use crate::block::Block;
use crate::constants::CHUNKSIZE;
use crate::positions::{ChunkPos, LocalBlockPos};
use crate::world_gen::basic::{floodfill_water, generate_empty_chunk, generate_landmass};
use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub blocks: Vec<Vec<Vec<Block>>>,
}

impl Chunk {
    pub fn generate(pos: &ChunkPos, seed: u32) -> Chunk {
        let mut chunk = generate_empty_chunk();
        generate_landmass(pos, seed, &mut chunk);
        floodfill_water(&mut chunk, pos);
        return chunk;
    }

    pub fn update(&mut self, dt: f32) -> bool {
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
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize] = block;
    }
    pub fn get_block(&self, pos: &LocalBlockPos) -> Option<&Block> {
        if pos.x < 0
            || pos.x >= (CHUNKSIZE) as i32
            || pos.y < 0
            || pos.y >= (CHUNKSIZE) as i32
            || pos.z < 0
            || pos.z >= (CHUNKSIZE) as i32
        {
            println!("couldn't get block at: {:?}", &pos);
            return None;
        }
        return Some(&self.blocks[pos.x as usize][pos.y as usize][pos.z as usize]);
    }
}
