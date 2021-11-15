use crate::blocks::block::{get_blockid, get_blocktype, BlockId};
use crate::blocks::block_type::BlockType;
use crate::world_gen::chunk::Chunk;
use noise::{Fbm, MultiFractal, NoiseFn, Seedable};
use std::cmp::max;
use vox_core::constants::{CHUNKSIZE, METACHUNKSIZE};
use vox_core::positions::{ChunkPos, LocalBlockPos};
use vox_core::utils::coord_to_array_indice;

pub struct ChunkGenerator {
    pub noise: Fbm,
    pub seed: u32,
    pub functions: Vec<fn(&ChunkGenerator, &ChunkPos, &mut Vec<BlockId>)>,
}

impl ChunkGenerator {
    pub fn new(seed: u32) -> ChunkGenerator {
        let mut functions = Vec::new();
        functions.push(generate_landmass as fn(&ChunkGenerator, &ChunkPos, &mut Vec<BlockId>));
        functions.push(floodfill_water as fn(&ChunkGenerator, &ChunkPos, &mut Vec<BlockId>));
        functions.push(plant_grass as fn(&ChunkGenerator, &ChunkPos, &mut Vec<BlockId>));
        ChunkGenerator {
            noise: Fbm::new()
                .set_seed(seed)
                .set_octaves(3)
                .set_persistence(0.6f64),
            seed: 1,
            functions,
        }
    }
    pub fn full_generation_pass(&self, pos: &ChunkPos) -> Vec<BlockId> {
        let mut chunk_data = generate_empty_chunk_data();
        for f in &self.functions {
            f(self, pos, &mut chunk_data);
        }
        return chunk_data;
    }
}
pub fn generate_empty_chunk_data() -> Vec<BlockId> {
    let mut arr: Vec<BlockId> = Vec::with_capacity(CHUNKSIZE * CHUNKSIZE * CHUNKSIZE);
    for _ in 0..CHUNKSIZE as i32 {
        for _ in 0..CHUNKSIZE as i32 {
            for _ in 0..CHUNKSIZE as i32 {
                arr.push(get_blockid( BlockType::Air));
            }
        }
    }
    return arr;
}

pub fn generate_landmass(chunk_generator: &ChunkGenerator, pos: &ChunkPos, chunk: &mut Vec<BlockId>) {
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            let height = get_xz_heigth(x, z, chunk_generator, pos);
            let height = height - pos.y * CHUNKSIZE as i32;
            for y in 0..height {
                if y >= CHUNKSIZE as i32 {
                    continue;
                }
                chunk[coord_to_array_indice(x as u32,y as u32,z as u32, CHUNKSIZE as u32)] = get_blockid(BlockType::Stone);
            }
        }
    }
}
pub fn plant_grass(chunk_generator: &ChunkGenerator, pos: &ChunkPos, chunk: &mut Vec<BlockId>) {
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            let height = get_xz_heigth(x, z, chunk_generator, pos);
            if height < (pos.y + 1) * CHUNKSIZE as i32 && height >= (pos.y) * CHUNKSIZE as i32 {
                let y = height - pos.y * CHUNKSIZE as i32;
                if height > (CHUNKSIZE as f32 * METACHUNKSIZE as f32 * 0.8) as i32 {
                    continue;
                }
                chunk[coord_to_array_indice(x as u32,y as u32,z as u32, CHUNKSIZE as u32)] = get_blockid(BlockType::Grass);
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

pub fn floodfill_water(_: &ChunkGenerator, pos: &ChunkPos, chunk: &mut Vec<BlockId>) {
    for x in 0..CHUNKSIZE as i32 {
        for z in 0..CHUNKSIZE as i32 {
            for y in 0..CHUNKSIZE as i32 {
                let water_level = CHUNKSIZE * METACHUNKSIZE / 3;
                let global_y = (y as i32 + (pos.y * CHUNKSIZE as i32)) as f64;
                if global_y < water_level as f64
                    && chunk[coord_to_array_indice(x as u32,y as u32,z as u32, CHUNKSIZE as u32)]
                        == get_blockid(BlockType::Air)
                {
                    chunk[coord_to_array_indice(x as u32,y as u32,z as u32, CHUNKSIZE as u32)] = get_blockid(BlockType::Water);
                }
            }
        }
    }
}
