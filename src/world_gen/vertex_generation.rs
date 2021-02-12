use crate::block::{Block, BlockSides, BlockType};
use crate::constants::{CHUNKSIZE, METACHUNKSIZE};
use crate::positions::{ChunkPos, GlobalBlockPos, LocalBlockPos};
use crate::renderer::vertex::Vertex;
use crate::world::World;
use std::time::Instant;

pub fn get_chunk_vertices(world: &World, chunk_pos: &ChunkPos) -> (Vec<Vertex>, Vec<u32>) {
    let chunk = world.get_chunk_unsafe(chunk_pos);
    if chunk.is_completely_air {
        return (Vec::new(), Vec::new());
    }

    let mut vertices: Vec<Vertex> = Vec::with_capacity(20000);
    let mut indices: Vec<u32> = Vec::with_capacity(20000);

    /*let mut block_pos = 0;
    let mut get_block = 0;
    let mut sides_to_render_t = 0;
    let mut get_mesh_t = 0;
    let mut increment_t = 0;
    let mut appending = 0;*/
    for x in 0..CHUNKSIZE as i32 {
        for y in 0..CHUNKSIZE as i32 {
            for z in 0..CHUNKSIZE as i32 {
                //let mut timer = Instant::now();
                let global_pos = GlobalBlockPos {
                    x: x + (chunk_pos.x * CHUNKSIZE as i32),
                    y: y + (chunk_pos.y * CHUNKSIZE as i32),
                    z: z + (chunk_pos.z * CHUNKSIZE as i32),
                };

                //block_pos += timer.elapsed().as_micros();
                //timer = Instant::now();

                let block = chunk.get_block_unsafe(&LocalBlockPos { x, y, z });
                if block.block_type == BlockType::Air {
                    continue;
                }

                //get_block += timer.elapsed().as_micros();
                //timer = Instant::now();

                let mut sides = sides_to_render(&world, &global_pos);

                //sides_to_render_t += timer.elapsed().as_micros();
                //timer = Instant::now();

                let (mut temp_vertices, mut temp_indices) = block.get_mesh(&global_pos, &sides);

                //get_mesh_t += timer.elapsed().as_micros();
                //timer = Instant::now();

                temp_indices = temp_indices
                    .iter()
                    .map(|i| i + (&vertices).len() as u32)
                    .collect();

                //increment_t += timer.elapsed().as_micros();
                //timer = Instant::now();

                {
                    vertices.append(&mut temp_vertices);
                    indices.append(&mut temp_indices);
                }
                //appending += timer.elapsed().as_micros();
                //timer = Instant::now();
            }
        }
    }
    /*println!(
        "{} {} {} {} {} {}",
        block_pos, get_block, sides_to_render_t, get_mesh_t, increment_t, appending
    );*/
    return (vertices, indices);
}
pub fn sides_to_render(world: &World, global_pos: &GlobalBlockPos) -> BlockSides {
    let mut sides = BlockSides::new();
    let mut reference_block = Block::new(BlockType::Air);
    let b = world.get_block(global_pos);
    if b.is_some() {
        reference_block = *b.unwrap();
    }
    if should_render_against_block(world, &global_pos.get_diff(1, 0, 0), &reference_block) {
        sides.right = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(-1, 0, 0), &reference_block) {
        sides.left = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(0, 1, 0), &reference_block) {
        sides.top = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(0, -1, 0), &reference_block) {
        sides.bot = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(0, 0, 1), &reference_block) {
        sides.back = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(0, 0, -1), &reference_block) {
        sides.front = true;
    }
    return sides;
}
#[inline]
pub fn should_render_against_block(
    world: &World,
    pos: &GlobalBlockPos,
    reference_block: &Block,
) -> bool {
    if pos.y == (METACHUNKSIZE * CHUNKSIZE) as i32 || pos.y < 0 {
        return true;
    }
    let block = world.get_block_unsafe(&pos);
    block.should_render_against(reference_block)
}
