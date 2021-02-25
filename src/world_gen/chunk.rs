use crate::blocks::block::{get_blocktype, BlockId};
use crate::blocks::block_type::BlockType;
use crate::constants::CHUNKSIZE;
use crate::positions::{ChunkPos, LocalBlockPos};
use crate::world_gen::basic::ChunkGenerator;
use log::warn;
use serde::{Deserialize, Serialize};
use serde_big_array::big_array;
use zerocopy::{AsBytes, FromBytes};

big_array! { BigArray; }

#[repr(C)]
#[derive(Serialize, Deserialize, Copy, Clone, FromBytes, AsBytes)]
pub struct ChunkData {
    #[serde(with = "BigArray")]
    pub d: [BlockId; CHUNKSIZE * CHUNKSIZE * CHUNKSIZE],
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub blocks: ChunkData,
    pub is_completely_air: bool,
}

impl Chunk {
    pub fn new(blocks: ChunkData) -> Chunk {
        Chunk {
            blocks,
            is_completely_air: false,
        }
    }
    pub fn generate(pos: &ChunkPos, seed: u32) -> Chunk {
        let chunk_generator = ChunkGenerator::new(seed);
        let mut chunk = chunk_generator.full_generation_pass(pos);
        for x in 0..CHUNKSIZE as i32 {
            for y in 0..CHUNKSIZE as i32 {
                for z in 0..CHUNKSIZE as i32 {
                    if get_blocktype(chunk.get_block(&LocalBlockPos { x, y, z }).unwrap())
                        != BlockType::Air
                    {
                        return chunk;
                    }
                }
            }
        }
        chunk.is_completely_air = true;
        return chunk;
    }

    pub fn update(&mut self, _dt: f32) -> bool {
        return false;
    }

    pub fn set_block(&mut self, block: BlockId, pos: &LocalBlockPos) {
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
        self.blocks.d[pos.x as usize
            + pos.y as usize * CHUNKSIZE as usize
            + pos.z as usize * CHUNKSIZE as usize * CHUNKSIZE as usize] = block;
    }
    pub fn get_block_unsafe(&self, pos: &LocalBlockPos) -> BlockId {
        self.blocks.d[pos.x as usize
            + pos.y as usize * CHUNKSIZE as usize
            + pos.z as usize * CHUNKSIZE as usize * CHUNKSIZE as usize]
    }
    pub fn get_block(&self, pos: &LocalBlockPos) -> Option<BlockId> {
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
            self.blocks.d[pos.x as usize
                + pos.y as usize * CHUNKSIZE as usize
                + pos.z as usize * CHUNKSIZE as usize * CHUNKSIZE as usize],
        );
    }
}
