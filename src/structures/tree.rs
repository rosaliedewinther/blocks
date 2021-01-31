use crate::block::{Block, BlockType};
use crate::positions::GlobalBlockPos;
use crate::world_gen::meta_chunk::MetaChunk;
use rand::distributions::{Distribution, Uniform};

pub fn place_tree(pos: &GlobalBlockPos, world: &mut MetaChunk) {
    let mut rng = rand::thread_rng();
    let height_range = Uniform::from(4..10);
    let height = height_range.sample(&mut rng);
    for y in 0..height {
        if y != height - 1 {
            world.set_block(&pos.get_diff(0, y, 0), Block::new(BlockType::Sand));
        }
        if y >= 2 {
            for x in -(height - y - 1)..height - y {
                for z in -(height - y - 1)..height - y {
                    let width = height_range.sample(&mut rng);
                    if width < 5 {
                        continue;
                    }
                    let currect_block = world.get_block(&pos.get_diff(x, y, z));
                    if currect_block.is_some()
                        && currect_block.unwrap().block_type == BlockType::Air
                    {
                        world.set_block(&pos.get_diff(x, y, z), Block::new(BlockType::Leaf));
                    }
                }
            }
        }
    }
}
