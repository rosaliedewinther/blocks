use crate::blocks::block::{get_blockid, get_blocktype};
use crate::blocks::block_type::BlockType;
use crate::world_gen::chunk::{Chunk, ChunkData};
use noise::{Fbm, MultiFractal, NoiseFn, Seedable};
use vox_core::constants::{CHUNKSIZE, METACHUNKSIZE};
use vox_core::positions::{ChunkPos, LocalBlockPos};

pub struct ChunkGenerator {
    pub noise: Fbm,
    pub seed: u32,
    pub functions: Vec<fn(&ChunkGenerator, &ChunkPos, &mut Chunk)>,
}

impl ChunkGenerator {
    pub fn new(seed: u32) -> ChunkGenerator {
        let mut functions = Vec::new();
        functions.push(generate_landmass as fn(&ChunkGenerator, &ChunkPos, &mut Chunk));
        functions.push(floodfill_water as fn(&ChunkGenerator, &ChunkPos, &mut Chunk));
        functions.push(plant_grass as fn(&ChunkGenerator, &ChunkPos, &mut Chunk));
        ChunkGenerator {
            noise: Fbm::new()
                .set_seed(seed)
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
    let empty = [get_blockid(BlockType::Air); CHUNKSIZE * CHUNKSIZE * CHUNKSIZE];
    /*let mut arr: Vec<Block> = Vec::with_capacity(CHUNKSIZE * CHUNKSIZE * CHUNKSIZE);
    for x in 0..CHUNKSIZE as i32 {
        for y in 0..CHUNKSIZE as i32 {
            for _ in 0..CHUNKSIZE as i32 {
                arr.push(block::Block::new(BlockType::Air));
            }
        }
    }*/
    return Chunk::new(ChunkData { d: empty });
}

pub fn generate_landmass(chunk_generator: &ChunkGenerator, pos: &ChunkPos, chunk: &mut Chunk) {
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            let height = get_xz_heigth(x, z, chunk_generator, pos);
            let height = height - pos.y * CHUNKSIZE as i32;
            for y in 0..height {
                if y >= CHUNKSIZE as i32 {
                    continue;
                }
                chunk.set_block(get_blockid(BlockType::Stone), &LocalBlockPos { x, y, z });
            }
        }
    }
}
pub fn plant_grass(chunk_generator: &ChunkGenerator, pos: &ChunkPos, chunk: &mut Chunk) {
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            let height = get_xz_heigth(x, z, chunk_generator, pos);
            if height < (pos.y + 1) * CHUNKSIZE as i32 && height >= (pos.y) * CHUNKSIZE as i32 {
                let y = height - pos.y * CHUNKSIZE as i32;
                if height > (CHUNKSIZE as f32 * METACHUNKSIZE as f32 * 0.8) as i32 {
                    continue;
                }
                chunk.set_block(get_blockid(BlockType::Grass), &LocalBlockPos { x, y, z });
            }
        }
    }
}

fn get_xz_heigth(x: i32, z: i32, chunk_generator: &ChunkGenerator, pos: &ChunkPos) -> i32 {
    let noise = [
        (x + (pos.x * CHUNKSIZE as i32)) as f64 / (METACHUNKSIZE * CHUNKSIZE) as f64,
        (z + (pos.z * CHUNKSIZE as i32)) as f64 / (METACHUNKSIZE * CHUNKSIZE) as f64,
    ];
    ((chunk_generator.noise.get(noise) + 1.0) * METACHUNKSIZE as f64 * CHUNKSIZE as f64 / 2.0)
        as i32
}

pub fn floodfill_water(_: &ChunkGenerator, pos: &ChunkPos, chunk: &mut Chunk) {
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            for y in 0..CHUNKSIZE as i32 {
                let water_level = CHUNKSIZE * METACHUNKSIZE / 3;
                let global_y = (y as i32 + (pos.y * CHUNKSIZE as i32)) as f64;
                if global_y < water_level as f64
                    && get_blocktype(chunk.get_block(&LocalBlockPos { x, y, z }).unwrap())
                        == BlockType::Air
                {
                    chunk.set_block(get_blockid(BlockType::Water), &LocalBlockPos { x, y, z });
                }
            }
        }
    }
}
