use crate::blocks::block::get_blockid;
use crate::blocks::block_type::BlockType;
use crate::world::big_world::BigWorld;
use vox_core::positions::GlobalBlockPos;

pub fn place_square(pos: &GlobalBlockPos, size: u32, world: &mut BigWorld) {
    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                world.set_block(
                    get_blockid(BlockType::Sand),
                    pos.get_diff(x as i32, y as i32, z as i32),
                );
            }
        }
    }
}
