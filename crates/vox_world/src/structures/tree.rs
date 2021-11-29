use crate::blocks::block::{get_blockid, get_blocktype};
use crate::blocks::block_type::BlockType;
use crate::world_gen::meta_chunk::MetaChunk;
use rand::distributions::{Distribution, Uniform};
use vox_core::positions::GlobalBlockPos;

pub fn place_tree(pos: &GlobalBlockPos, world: &mut MetaChunk) {
    let mut rng = rand::thread_rng();
    let height_range = Uniform::from(8..12);
    let height = height_range.sample(&mut rng);
    for y in 0..height {
        if y < height - 2 {
            world.set_block(&pos.get_diff(0, y, 0), get_blockid(BlockType::Sand));
        }
        if y >= 4 {
            for x in -(height - y - 1)..height - y {
                for z in -(height - y - 1)..height - y {
                    let currect_block = world.get_block(&pos.get_diff(x, y, z));
                    if get_blocktype(currect_block) == BlockType::Air
                    {
                        world.set_block(&pos.get_diff(x, y, z), get_blockid(BlockType::Leaf));
                    }
                }
            }
        }
    }
}
