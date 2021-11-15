use crate::blocks::block::{get_blocktype, BlockId, get_blockid};
use crate::blocks::block_type::BlockType;
use crate::world_gen::basic::ChunkGenerator;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use serde_big_array::big_array;
use vox_core::constants::CHUNKSIZE;
use vox_core::positions::{ChunkPos, LocalBlockPos};
use vox_core::utils::coord_to_array_indice;

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub blocks: Vec<BlockId>,
    pub is_completely_air: bool,
}

impl Chunk {
    pub fn generate(chunk_generator: &ChunkGenerator, pos: &ChunkPos) -> Chunk {
        let mut chunk_data = chunk_generator.full_generation_pass(pos);
        for x in 0..CHUNKSIZE as i32 {
            for y in 0..CHUNKSIZE as i32 {
                for z in 0..CHUNKSIZE as i32 {
                    if chunk_data[coord_to_array_indice(x as u32,y as u32,z as u32, CHUNKSIZE as u32)] != get_blockid(BlockType::Air)
                    {
                        return Chunk{ blocks: chunk_data, is_completely_air: false };
                    }
                }
            }
        }
        return Chunk{blocks: vec![], is_completely_air: true};
    }

    pub fn update(&mut self, _dt: f32) -> bool {
        return false;
    }

    pub fn set_block(&mut self, block: BlockId, pos: &LocalBlockPos) {
        if self.is_completely_air{
            self.blocks = vec![get_blockid(BlockType::Air); CHUNKSIZE * CHUNKSIZE * CHUNKSIZE ];
            self.is_completely_air = false;
        }
        if pos.x < 0
            || pos.x > (CHUNKSIZE - 1) as i32
            || pos.y < 0
            || pos.y > (CHUNKSIZE - 1) as i32
            || pos.z < 0
            || pos.z > (CHUNKSIZE - 1) as i32
        {
            println!("couldn't set block at: {:?}", &pos);
            return;
        }
        self.blocks[coord_to_array_indice(pos.x as u32, pos.y as u32, pos.z as u32, CHUNKSIZE as u32)] = block;
    }
    pub fn get_block(&self, pos: &LocalBlockPos) -> Option<BlockId> {
        if self.is_completely_air{
            return Some(get_blockid(BlockType::Air))
        }
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
        return Some(
            self.blocks[coord_to_array_indice(pos.x as u32, pos.y as u32, pos.z as u32, CHUNKSIZE as u32)],
        );
    }
}
