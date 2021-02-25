use crate::blocks::block::get_blockid;
use crate::blocks::block_type::BlockType;
use crate::positions::GlobalBlockPos;
use crate::world_gen::meta_chunk::MetaChunk;

pub fn place_square(pos: &GlobalBlockPos, size: u32, world: &mut MetaChunk) {
    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                world.set_block(
                    &pos.get_diff(x as i32, y as i32, z as i32),
                    get_blockid(BlockType::Sand),
                );
            }
        }
    }
}
