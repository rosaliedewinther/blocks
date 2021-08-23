use crate::algorithms::noise_abstraction::Noise;
use crate::blocks::block::{get_blockid, BlockId};
use crate::blocks::block_type::BlockType;
use crate::world_gen::generator::WorldGenerator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator};
use std::time::Instant;
use vox_core::constants::WORLD_SIZE;
use wgpu::Instance;

pub struct StandardWorldGenerator<T: Noise + std::marker::Sync> {
    noise: T,
}

impl<T: Noise + std::marker::Sync> WorldGenerator for StandardWorldGenerator<T> {
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
        let timer = Instant::now();
        let world_data: Vec<BlockId> = (0..size * size * size)
            .into_par_iter()
            .enumerate()
            .map(|(index, val)| {
                let x = (index % size) as f32;
                let y = ((index / size) % size) as f32;
                let z = (index / (size * size) % size) as f32;

                let noise_data = self.noise.get(
                    x / (WORLD_SIZE) as f32,
                    y / (WORLD_SIZE) as f32,
                    z / (WORLD_SIZE) as f32,
                );
                if noise_data > 0.3 {
                    return ((x as i32 % 8) + 1) as BlockId;
                }
                return 0u8;
            })
            .collect();
        println!(
            "generated world in {:?} seconds",
            timer.elapsed().as_secs_f64()
        );
        return world_data.into_boxed_slice();
    }

    fn add_generation_layer(&self, generation_function: fn(i32, i32, i32, usize)) {
        todo!()
    }
}
