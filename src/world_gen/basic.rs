use crate::block;
use crate::block::{Block, BlockType};
use crate::constants::{CHUNKSIZE, VERTICALCHUNKS};
use crate::positions::ChunkPos;
use crate::world_gen::chunk::Chunk;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin, Seedable};

pub struct ChunkGenerator {
    pub noise: Fbm,
    pub seed: u32,
    pub functions: Vec<fn(&ChunkGenerator, &ChunkPos, &mut Chunk)>,
}

impl ChunkGenerator {
    pub fn new() -> ChunkGenerator {
        let mut functions = Vec::new();
        functions.push(generate_landmass as fn(&ChunkGenerator, &ChunkPos, &mut Chunk));
        functions.push(floodfill_water as fn(&ChunkGenerator, &ChunkPos, &mut Chunk));
        ChunkGenerator {
            noise: Fbm::new()
                .set_seed(1)
                .set_octaves(3)
                .set_persistence(0.6f64),
            seed: 1,
            functions,
        }
    }
    pub fn full_generation_pass(&self, pos: &ChunkPos) -> Chunk {
        let mut chunk = generate_empty_chunk();
        for f in &self.functions {
            f(self, pos, &mut chunk);
        }
        return chunk;
    }
}
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

pub fn generate_landmass(chunk_generator: &ChunkGenerator, pos: &ChunkPos, chunk: &mut Chunk) {
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            let perlin_input = [
                (x + (pos.x * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                (z + (pos.z * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
            ];
            let height = chunk_generator.noise.get(perlin_input);
            for y in 0..CHUNKSIZE as i32 {
                let perlin_input2 = [
                    (x + (pos.x * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                    (z + (pos.z * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                    (y + (pos.y * CHUNKSIZE as i32)) as f64 / (VERTICALCHUNKS * CHUNKSIZE) as f64,
                ];
                let addition = (chunk_generator.noise.get(perlin_input2) + 1f64) / 2f64;
                let global_y = (y as i32 + (pos.y * CHUNKSIZE as i32)) as f64;
                if (addition * height * VERTICALCHUNKS as f64 * CHUNKSIZE as f64 / 2f64
                    + VERTICALCHUNKS as f64 * CHUNKSIZE as f64 / 2f64)
                    >= global_y
                {
                    if global_y
                        < CHUNKSIZE as f64 * VERTICALCHUNKS as f64
                            - CHUNKSIZE as f64 * (VERTICALCHUNKS as f64 / 2.0)
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

pub fn floodfill_water(chunk_generator: &ChunkGenerator, pos: &ChunkPos, chunk: &mut Chunk) {
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            for y in 0..CHUNKSIZE as i32 {
                let water_level = CHUNKSIZE * VERTICALCHUNKS / 3;
                let global_y = (y as i32 + (pos.y * CHUNKSIZE as i32)) as f64;
                if global_y < water_level as f64
                    && chunk.blocks[x as usize][y as usize][z as usize].block_type == BlockType::Air
                {
                    chunk.blocks[x as usize][y as usize][z as usize] =
                        block::Block::new(BlockType::Water);
                }
            }
        }
    }
}
