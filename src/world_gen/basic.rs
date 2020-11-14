use crate::block;
use crate::block::{Block, BlockType};
use crate::constants::{CHUNKSIZE, VERTICALCHUNKS};
use crate::positions::ChunkPos;
use crate::world_gen::chunk::Chunk;
use noise::{NoiseFn, Perlin, Seedable};

pub fn generate_empty_chunk() -> Chunk {
    let mut arr: Vec<Vec<Vec<Block>>> = Vec::new();
    for x in 0..CHUNKSIZE as i32 {
        arr.push(Vec::with_capacity(CHUNKSIZE));
        for y in 0..CHUNKSIZE as i32 {
            arr[x as usize].push(Vec::with_capacity(CHUNKSIZE));
            for _ in 0..CHUNKSIZE as i32 {
                arr[x as usize][y as usize].push(block::Block::new(BlockType::Air));
            }
        }
    }
    return Chunk { blocks: arr };
}

pub fn generate_landmass(pos: &ChunkPos, seed: u32, chunk: &mut Chunk) {
    let perlin = Perlin::new();
    let perlin2 = Perlin::new();
    perlin.set_seed(seed);
    perlin2.set_seed(seed + i32::max_value() as u32);
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            let perlin_input = [
                (x + (pos.x * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                (z + (pos.z * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
            ];
            let height = perlin.get(perlin_input);
            for y in 0..CHUNKSIZE as i32 {
                let perlin_input2 = [
                    (x + (pos.x * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                    (z + (pos.z * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                    (y + (pos.y * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                ];
                let addition = (perlin.get(perlin_input2) + 1f64) / 2f64;
                let global_y = (y as i32 + (pos.y * CHUNKSIZE as i32)) as f64;
                if (addition * height * VERTICALCHUNKS as f64 * CHUNKSIZE as f64 / 2f64
                    + VERTICALCHUNKS as f64 * CHUNKSIZE as f64 / 2f64)
                    >= global_y
                {
                    if global_y
                        < CHUNKSIZE as f64 * VERTICALCHUNKS as f64
                            - CHUNKSIZE as f64 * (VERTICALCHUNKS as f64 - 3f64)
                    {
                        chunk.blocks[x as usize][y as usize][z as usize] =
                            block::Block::new(BlockType::Grass);
                    } else if global_y < CHUNKSIZE as f64 * VERTICALCHUNKS as f64 {
                        chunk.blocks[x as usize][y as usize][z as usize] =
                            block::Block::new(BlockType::Stone);
                    } else {
                        chunk.blocks[x as usize][y as usize][z as usize] = block::Block::rand_new();
                    }
                }
            }
        }
    }
}

pub fn floodfill_water(chunk: &mut Chunk, pos: &ChunkPos) {
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            for y in 0..CHUNKSIZE as i32 {
                let global_y = (y as i32 + (pos.y * CHUNKSIZE as i32)) as f64;
                if global_y < CHUNKSIZE as f64
                    && chunk.blocks[x as usize][y as usize][z as usize].block_type == BlockType::Air
                {
                    chunk.blocks[x as usize][y as usize][z as usize] =
                        block::Block::new(BlockType::Water);
                }
            }
        }
    }
}
