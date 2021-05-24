use crate::blocks::block::{get_blocktype, get_mesh, should_render_against, BlockId};
use crate::blocks::block_type::BlockType;
use crate::blocks::blockside::BlockSides;
use crate::world::small_world::SmallWorld;
use crate::world_gen::chunk::Chunk;
use std::time::Instant;
use vox_core::constants::{CHUNKSIZE, METACHUNKSIZE};
use vox_core::positions::{ChunkPos, GlobalBlockPos, LocalBlockPos};
use vox_render::renderer::vertex::Vertex;

pub fn get_chunk_vertices(world: &SmallWorld, chunk_pos: &ChunkPos) -> (Vec<Vertex>, Vec<u32>) {
    return match world.get_chunk(chunk_pos) {
        None => (Vec::new(), Vec::new()),
        Some(chunk) => {
            if chunk.is_completely_air {
                return (Vec::new(), Vec::new());
            }
            let mut transparant_vertices: Vec<Vertex> = Vec::with_capacity(10000);
            let mut transparant_indices: Vec<u32> = Vec::with_capacity(10000);

            let mut opaque_vertices: Vec<Vertex> = Vec::with_capacity(20000);
            let mut opaque_indices: Vec<u32> = Vec::with_capacity(20000);

            for x in 0..CHUNKSIZE as i32 {
                for y in 0..CHUNKSIZE as i32 {
                    for z in 0..CHUNKSIZE as i32 {
                        //let mut timer = Instant::now();
                        let global_pos = GlobalBlockPos {
                            x: x + (chunk_pos.x * CHUNKSIZE as i32),
                            y: y + (chunk_pos.y * CHUNKSIZE as i32),
                            z: z + (chunk_pos.z * CHUNKSIZE as i32),
                        };
                        let block = chunk.get_block(&LocalBlockPos { x, y, z }).unwrap();
                        if get_blocktype(block) == BlockType::Air {
                            continue;
                        }
                        let mut sides = sides_to_render(world, &global_pos);
                        if sides.is_all(false) {
                            continue;
                        }
                        let (mut temp_vertices, mut temp_indices) =
                            get_mesh(block, &global_pos, &sides);
                        if get_blocktype(block) == BlockType::Water {
                            temp_indices = temp_indices
                                .iter()
                                .map(|i| i + (&transparant_vertices).len() as u32)
                                .collect();
                            {
                                transparant_vertices.append(&mut temp_vertices);
                                transparant_indices.append(&mut temp_indices);
                            }
                        } else {
                            temp_indices = temp_indices
                                .iter()
                                .map(|i| i + (&opaque_vertices).len() as u32)
                                .collect();
                            {
                                opaque_vertices.append(&mut temp_vertices);
                                opaque_indices.append(&mut temp_indices);
                            }
                        }
                    }
                }
            }
            transparant_indices = transparant_indices
                .iter()
                .map(|i| i + (&opaque_vertices).len() as u32)
                .collect();
            opaque_vertices.extend(transparant_vertices.into_iter());

            opaque_indices.extend(transparant_indices.into_iter());
            (opaque_vertices, opaque_indices)
        }
    };
}
pub fn sides_to_render(world: &SmallWorld, global_pos: &GlobalBlockPos) -> BlockSides {
    let mut sides = BlockSides::new();
    let mut reference_block = world.get_block(*global_pos).unwrap();
    if should_render_against_block(world, &global_pos.get_diff(1, 0, 0), reference_block) {
        sides.right = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(-1, 0, 0), reference_block) {
        sides.left = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(0, 1, 0), reference_block) {
        sides.top = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(0, -1, 0), reference_block) {
        sides.bot = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(0, 0, 1), reference_block) {
        sides.back = true;
    }
    if should_render_against_block(world, &global_pos.get_diff(0, 0, -1), reference_block) {
        sides.front = true;
    }
    return sides;
}
#[inline]
pub fn should_render_against_block(
    world: &SmallWorld,
    pos: &GlobalBlockPos,
    reference_block: BlockId,
) -> bool {
    if pos.y == (METACHUNKSIZE * CHUNKSIZE) as i32 || pos.y < 0 {
        return true;
    }
    return match world.get_block(*pos) {
        None => true,
        Some(b) => should_render_against(reference_block, b),
    };
}
