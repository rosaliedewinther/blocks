use crate::blocks::block::{get_blocktype, BlockId, get_blockid};
use crate::blocks::block_type::BlockType;
use crate::world_gen::basic::ChunkGenerator;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use serde_big_array::big_array;
use vox_core::constants::CHUNKSIZE;
use vox_core::positions::{ChunkPos, LocalBlockPos};
use vox_core::utils::coord_to_array_indice;
use arrayvec::ArrayVec;

#[derive(Serialize, Deserialize, Copy, Clone)]
enum BlockIdOrPointer{
    Id(BlockId),
    Ptr(u32)
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    single_block: BlockId,
    top_layer: [BlockIdOrPointer;64], //4^3
    first_medium_layer: Vec<[BlockIdOrPointer;4096]>, //8^3
    second_medium_layer: Vec<[BlockIdOrPointer;8]>, //2^3
    bottom_layer: Vec<[BlockId;8]>, //2^3
}

impl Chunk {
    pub fn generate(chunk_generator: &ChunkGenerator, pos: &ChunkPos) -> Chunk {
        let mut c = Chunk{
            single_block: get_blockid(BlockType::Air),
            top_layer: [BlockIdOrPointer::Id(get_blockid(BlockType::Air)); 64],
            first_medium_layer: Vec::new(),
            second_medium_layer: Vec::new(),
            bottom_layer: Vec::new()
        };

        return c;
    }

    pub fn update(&mut self, _dt: f32) -> bool {
        return false;
    }

    pub fn set_block(&mut self, block: BlockId, pos: &LocalBlockPos) {
        debug_assert!(pos.x < 0
            || pos.x > (CHUNKSIZE - 1) as i32
            || pos.y < 0
            || pos.y > (CHUNKSIZE - 1) as i32
            || pos.z < 0
            || pos.z > (CHUNKSIZE - 1) as i32);
        match self.single_block{
            255 => {
                self.set_block_l1(block, pos);
            },
            b => {
                if b == block{
                    return
                } else {
                    self.top_layer = [BlockIdOrPointer::Id(*b);64];
                    self.single_block = get_blockid(BlockType::Unknown);
                    self.set_block_l1(block, pos);
                    if self.top_layer.iter().all(|e| e == BlockIdOrPointer::Id && e == self.top_layer[0]){
                        match self.top_layer[0]{
                            BlockIdOrPointer::Id(id) => self.single_block = id,
                            BlockIdOrPointer::Ptr(_) => panic!("something went horribly wrong when setting a block")
                        }
                    }
                }
            }
        }
    }
    pub fn set_block_l1(&mut self, block: BlockId, pos: &LocalBlockPos){
        let index = coord_to_array_indice((pos.x / 32) as u32, (pos.y / 32) as u32, (pos.z / 32) as u32, 4);
        todo!("check everything");
        match self.top_layer[index] {
            BlockIdOrPointer::Id(id) => {
                if id == block{
                    return
                } else {
                    self.top_layer = [BlockIdOrPointer::Id(*b);64];
                    self.single_block = get_blockid(BlockType::Unknown);
                    self.set_block_l1(block, pos);
                    if self.top_layer.iter().all(|e| e == BlockIdOrPointer::Id && e == self.top_layer[0]){
                        match self.top_layer[0]{
                            BlockIdOrPointer::Id(id) => self.single_block = id,
                            BlockIdOrPointer::Ptr(_) => panic!("something went horribly wrong when setting a block")
                        }
                    }
                }
            }
            BlockIdOrPointer::Ptr(ptr) => {
                if ptr < self.first_medium_layer.len() as u32 {
                    self.set_block_l2(block: BlockId, pos: &LocalBlockPos, ptr);
                }
            }
        }
    }
    pub fn set_block_l2(&mut self, block: BlockId, pos: &LocalBlockPos, ptr: u32){
        todo!("completely write");
    }

    pub fn get_block(&self, pos: &LocalBlockPos) -> BlockId {
        debug_assert!(pos.x >= 0
            || pos.x <= (CHUNKSIZE - 1) as i32
            || pos.y >= 0
            || pos.y <= (CHUNKSIZE - 1) as i32
            || pos.z >= 0
            || pos.z <= (CHUNKSIZE - 1) as i32);
        return match self.single_block{
            255 => self.get_block_l1(pos),
            b => b
        }
    }
    pub fn get_block_l1(&self, pos: &LocalBlockPos) -> BlockId{
        let index = coord_to_array_indice((pos.x / 32) as u32, (pos.y / 32) as u32, (pos.z / 32) as u32, 4);
        return match self.top_layer[index] {
            BlockIdOrPointer::Id(id) => id,
            BlockIdOrPointer::Ptr(ptr) => self.get_block_l2(pos, ptr)
        }
    }
    pub fn get_block_l2(&self, pos: &LocalBlockPos, ptr: u32) -> BlockId{
        let index = coord_to_array_indice(((pos.x%32)/4) as u32, ((pos.y%32)/4) as u32, ((pos.z%32)/4) as u32, 8);
        return match self.first_medium_layer[ptr][index] {
            BlockIdOrPointer::Id(id) => id,
            BlockIdOrPointer::Ptr(ptr) => self.get_block_l3(pos, ptr)
        }
    }
    pub fn get_block_l3(&self, pos: &LocalBlockPos, ptr: u32) -> BlockId{
        let index = coord_to_array_indice(((pos.x%4)/8) as u32, ((pos.y%4)/8) as u32, ((pos.z%4)/8) as u32, 2);
        return match self.second_medium_layer[ptr][index] {
            BlockIdOrPointer::Id(id) => id,
            BlockIdOrPointer::Ptr(ptr) => self.get_block_l4(pos, ptr)
        }
    }
    pub fn get_block_l4(&self, pos: &LocalBlockPos, ptr: u32) -> BlockId{
        let index = coord_to_array_indice((pos.x % 2) as u32, (pos.y % 2) as u32, (pos.z % 2) as u32, 2);
        return self.bottom_layer[ptr][index]
    }
}
