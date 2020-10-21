use crate::block;
use crate::block::{Block, BlockType};
use crate::constants::{CHUNKSIZE, VERTICALCHUNKS};
use crate::positions::{ChunkPos, LocalBlockPos};
use log::warn;
use noise::{NoiseFn, Perlin, Seedable};

#[derive(Debug, Clone)]
pub struct BlockSides {
    pub top: bool,
    pub bot: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}

impl BlockSides {
    pub fn new() -> BlockSides {
        BlockSides {
            top: false,
            bot: false,
            left: false,
            right: false,
            front: false,
            back: false,
        }
    }
}

pub struct Chunk {
    pub blocks: Vec<Vec<Vec<Block>>>,
}

impl Chunk {
    pub fn generate(pos: &ChunkPos, seed: &u32) -> Chunk {
        let mut arr: Vec<Vec<Vec<Block>>> = Vec::new();
        for x in 0..CHUNKSIZE as i32 {
            arr.push(Vec::with_capacity(CHUNKSIZE));
            for y in 0..CHUNKSIZE as i32 {
                arr[x as usize].push(Vec::with_capacity(CHUNKSIZE));
                for z in 0..CHUNKSIZE as i32 {
                    arr[x as usize][y as usize].push(block::Block::new(BlockType::Air));
                }
            }
        }
        let perlin = Perlin::new();
        perlin.set_seed(*seed);
        for x in 0..CHUNKSIZE as i32 {
            for z in 0..CHUNKSIZE as i32 {
                let perlin_input = [
                    (x + (pos.x * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                    (z + (pos.z * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                ];
                let height = perlin.get(perlin_input);
                for y in 0..CHUNKSIZE as i32 {
                    let global_y = ((y as i32 + (pos.y * CHUNKSIZE as i32)) as f64);
                    if (height * VERTICALCHUNKS as f64 * CHUNKSIZE as f64 / 2f64
                        + VERTICALCHUNKS as f64 * CHUNKSIZE as f64 / 2f64)
                        >= global_y
                    {
                        if global_y < CHUNKSIZE as f64 {
                            arr[x as usize][y as usize][z as usize] =
                                block::Block::new(BlockType::Water);
                        } else if global_y
                            < CHUNKSIZE as f64 * VERTICALCHUNKS as f64
                                - CHUNKSIZE as f64 * (VERTICALCHUNKS as f64 - 3f64)
                        {
                            arr[x as usize][y as usize][z as usize] =
                                block::Block::new(BlockType::Grass);
                        } else if global_y < CHUNKSIZE as f64 * VERTICALCHUNKS as f64 {
                            arr[x as usize][y as usize][z as usize] =
                                block::Block::new(BlockType::Stone);
                        } else {
                            arr[x as usize][y as usize][z as usize] = block::Block::rand_new();
                        }
                    }
                }
            }
        }

        let c = Chunk { blocks: arr };
        return c;
    }

    pub fn update(&mut self, dt: &f32) -> bool {
        return false;
    }

    pub fn get_blocktype(&self, pos: &LocalBlockPos) -> BlockType {
        let maybe_block_type = self.get_block(pos);
        if maybe_block_type.is_none() {
            return BlockType::Air;
        }
        return maybe_block_type.unwrap().block_type;
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
            || pos.x > (CHUNKSIZE - 1) as i32
            || pos.y < 0
            || pos.y > (CHUNKSIZE - 1) as i32
            || pos.z < 0
            || pos.z > (CHUNKSIZE - 1) as i32
        {
            return None;
        }
        return Some(&self.blocks[pos.x as usize][pos.y as usize][pos.z as usize]);
    }
}
