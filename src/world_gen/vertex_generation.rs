use crate::block::{Block, BlockSides, BlockType};
use crate::constants::CHUNKSIZE;
use crate::positions::{ChunkPos, GlobalBlockPos, LocalBlockPos};
use crate::renderer::vertex::Vertex;
use crate::world::World;

pub fn get_chunk_vertices(world: &World, chunk_pos: &ChunkPos) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::with_capacity(20000);
    let mut indices: Vec<u32> = Vec::with_capacity(20000);
    let chunk = world.get_chunk(chunk_pos).unwrap();
    for x in 0..CHUNKSIZE as i32 {
        for y in 0..CHUNKSIZE as i32 {
            for z in 0..CHUNKSIZE as i32 {
                let global_pos = GlobalBlockPos {
                    x: x + (chunk_pos.x * CHUNKSIZE as i32),
                    y: y + (chunk_pos.y * CHUNKSIZE as i32),
                    z: z + (chunk_pos.z * CHUNKSIZE as i32),
                };

                let block = chunk.get_block(&LocalBlockPos { x, y, z });
                if block.is_some() && block.unwrap().block_type == BlockType::Air {
                    continue;
                }
                let sides = sides_to_render(&world, &global_pos);

                let block: &Block = &chunk.get_block(&LocalBlockPos { x, y, z }).unwrap();
                let (mut temp_vertices, mut temp_indices) = block.get_mesh(&global_pos, &sides);
                temp_indices = temp_indices
                    .iter()
                    .map(|i| i + (&vertices).len() as u32)
                    .collect();
                {
                    vertices.append(&mut temp_vertices);
                    indices.append(&mut temp_indices);
                }
            }
        }
    }
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
pub fn should_render_against_block(
    world: &World,
    pos: &GlobalBlockPos,
    reference_block: &Block,
) -> bool {
    let block = world.get_block(&pos);
    match block {
        Some(b) => b.should_render_against(reference_block),
        None => true,
    }
}
