#![allow(dead_code)]

use crate::game::VoxGame;
use crate::logger::setup_logger;
use std::time::Instant;
use vox_core::positions::{ChunkPos, LocalBlockPos, LocalChunkPos};
use vox_core::utils::coord_to_array_indice;
use vox_world::blocks::block::{get_blockid, BlockId};
use vox_world::blocks::block_type::BlockType;
use vox_world::world_gen::basic::ChunkGenerator;
use vox_world::world_gen::chunk::{Chunk, OldChunk};

mod game;
mod logger;
mod personal_world;
mod ui;

fn main() {
    setup_logger().unwrap();

    let sizes = 64;
    let g = ChunkGenerator::new(0);
    let timer = Instant::now();
    let mut c = Chunk::<6, 2, 8>::generate(&g, &ChunkPos { x: 0, y: 0, z: 0 });
    for x in 0..sizes {
        for y in 0..sizes {
            for z in 0..sizes {
                if rand::random::<f32>() < 0.01 {
                    continue;
                }
                c.set_block(
                    get_blockid(BlockType::Grass),
                    &LocalBlockPos { x: x, y: y, z: z },
                );
            }
        }
    }

    let time = timer.elapsed().as_secs_f32();
    println!(
        "new structure size with chunksize 2: {:?} in {} seconds",
        c.get_structure_size(),
        time
    );
    let sizes = 64;
    let timer = Instant::now();
    let mut c = Chunk::<3, 4, 64>::generate(&g, &ChunkPos { x: 0, y: 0, z: 0 });
    for x in 0..sizes {
        for y in 0..sizes {
            for z in 0..sizes {
                if rand::random::<f32>() < 0.01 {
                    continue;
                }
                c.set_block(
                    get_blockid(BlockType::Grass),
                    &LocalBlockPos { x: x, y: y, z: z },
                );
            }
        }
    }

    let time = timer.elapsed().as_secs_f32();
    println!(
        "new structure size with chunksize 4: {:?} in {} seconds",
        c.get_structure_size(),
        time
    );

    let sizes = 64;
    let g = ChunkGenerator::new(0);
    let timer = Instant::now();
    let mut c = Chunk::<2, 8, 8>::generate(&g, &ChunkPos { x: 0, y: 0, z: 0 });
    for x in 0..sizes {
        for y in 0..sizes {
            for z in 0..sizes {
                if rand::random::<f32>() < 0.01 {
                    continue;
                }
                c.set_block(
                    get_blockid(BlockType::Grass),
                    &LocalBlockPos { x: x, y: y, z: z },
                );
            }
        }
    }

    let time = timer.elapsed().as_secs_f32();
    println!(
        "new structure size with chunksize 8: {:?} in {} seconds",
        c.get_structure_size(),
        time
    );
    let sizes = 64;
    let timer = Instant::now();
    let mut old = OldChunk {
        data: vec![get_blockid(BlockType::Air); (sizes * sizes * sizes) as usize],
    };
    for x in 0..sizes {
        for y in 0..sizes {
            for z in 0..sizes {
                if rand::random::<f32>() < 0.01 {
                    continue;
                }
                old.data[coord_to_array_indice(x as u32, y as u32, z as u32, sizes as u32)] =
                    get_blockid(BlockType::Grass);
            }
        }
    }
    let time = timer.elapsed().as_secs_f32();
    println!(
        "size of old structure: {} in {} seconds",
        std::mem::size_of::<OldChunk>() + old.data.len() * std::mem::size_of::<BlockId>(),
        time
    );
}
