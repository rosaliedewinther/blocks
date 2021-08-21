use crate::algorithms::noise_abstraction::Noise;
use crate::blocks::block::{get_blockid, BlockId};
use crate::blocks::block_type::BlockType;
use crate::world_gen::generator::WorldGenerator;
use vox_core::constants::{BRICKMAPSIZE, BRICKSIZE};

pub struct StandardWorldGenerator<T: Noise> {
    noise: T,
}

impl<T: Noise> WorldGenerator for StandardWorldGenerator<T> {
    fn new() -> Self {
        Self {
            noise: T::new(1, 0, 6.0),
        }
    }

    fn generate_area(
        &self,
        x_start: i32,
        y_start: i32,
        z_start: i32,
        size: usize,
    ) -> Box<[BlockId]> {
        let mut world_data = Vec::with_capacity(size * size * size);
        for x in x_start..x_start + size as i32 {
            for y in y_start..y_start + size as i32 {
                for z in z_start..z_start + size as i32 {
                    let noise_data = self.noise.get(
                        x as f32 / (BRICKMAPSIZE * BRICKSIZE * 3) as f32,
                        y as f32 / (BRICKMAPSIZE * BRICKSIZE * 3) as f32,
                        z as f32 / (BRICKMAPSIZE * BRICKSIZE * 3) as f32,
                    );
                    let block = if noise_data > 0.3 {
                        ((z % 8) + 1) as BlockId
                    } else {
                        get_blockid(BlockType::Air)
                    };
                    world_data.push(block);
                }
            }
        }
        return world_data.into_boxed_slice();
    }

    fn add_generation_layer(&self, generation_function: fn(i32, i32, i32, usize)) {
        todo!()
    }
}
