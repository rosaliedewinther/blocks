#![allow(dead_code)]

use std::time::Instant;
use vox_core::positions::{ChunkPos, LocalBlockPos, LocalChunkPos};
use vox_core::utils::coord_to_array_indice;
use vox_world::blocks::block::{BlockId, get_blockid};
use vox_world::blocks::block_type::BlockType;
use vox_world::world_gen::basic::ChunkGenerator;
use vox_world::world_gen::chunk::{Chunk, OldChunk};
use crate::game::VoxGame;
use crate::logger::setup_logger;

mod game;
mod logger;
mod personal_world;
mod ui;

fn main() {
    setup_logger().unwrap();


    let g = ChunkGenerator::new(0);
    let timer = Instant::now();
    let mut c = Chunk::<4,2,8>::generate(&g, &ChunkPos { x: 0, y: 0, z: 0 });
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                println!("{} {} {}", x,y,z);
                c.set_block(get_blockid(BlockType::Grass), &LocalBlockPos{
                    x: x,
                    y: y,
                    z: z
                });
            }
        }
    }
    let time = timer.elapsed().as_secs_f32();
    println!("new structure size with chunksize 2: {:?}", c.get_structure_size());
    println!("in {} seconds", time);

    let timer = Instant::now();
    let mut c = Chunk::<2,4,64>::generate(&g, &ChunkPos { x: 0, y: 0, z: 0 });
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                c.set_block(get_blockid(BlockType::Grass), &LocalBlockPos{
                    x: x,
                    y: y,
                    z: z
                });
            }
        }
    }
    c.print_structured();
    let time = timer.elapsed().as_secs_f32();
    println!("new structure size with chunksize 4: {:?}", c.get_structure_size());
    println!("in {} seconds", time);

    let timer = Instant::now();
    let mut old = OldChunk{ data: vec![get_blockid(BlockType::Air);16*16*16] };
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                old.data[coord_to_array_indice(x as u32, y as u32, z as u32, 16)] = get_blockid(BlockType::Grass);
            }
        }
    }
    let time = timer.elapsed().as_secs_f32();
    println!("size of old structure: {}", std::mem::size_of::<OldChunk>() + old.data.len()*std::mem::size_of::<BlockId>());
    println!("in {} seconds", time);


    //let mut game = VoxGame::new();
    //game.run();
}
