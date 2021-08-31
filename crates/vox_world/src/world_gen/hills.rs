use crate::algorithms::noise_abstraction::Noise;
use crate::blocks::block::BlockId;
use crate::world_gen::generator::WorldGenerator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::time::Instant;
use vox_core::constants::WORLD_SIZE;

pub struct HillsWorldGenerator<T: Noise + std::marker::Sync> {
    noise: T,
}

impl<T: Noise + std::marker::Sync> WorldGenerator for HillsWorldGenerator<T> {
    fn new() -> Self {
        Self {
            noise: T::new(1, 0, 6.0),
        }
    }

    fn generate_area(
        &self,
        x_start: i32,
        _y_start: i32,
        z_start: i32,
        size: usize,
    ) -> Box<[BlockId]> {
        let timer = Instant::now();
        let heigthmap: Vec<f32> = (0..size * size)
            .into_par_iter()
            .enumerate()
            .map(|(index, _val)| {
                let x = (index % size) as f32;
                let z = ((index / size) % size) as f32;

                let noise_data = self.noise.get(
                    x_start as f32 + x / (WORLD_SIZE) as f32,
                    0.0,
                    z_start as f32 + z / (WORLD_SIZE) as f32,
                );
                return noise_data;
            })
            .collect();
        let world_data: Vec<BlockId> = (0..size * size * size)
            .into_par_iter()
            .enumerate()
            .map(|(index, _val)| {
                let x = index % size;
                let y = (index / size) % size;
                let z = index / (size * size) % size;

                if ((heigthmap[x + z * size] as f64 + 1.0) / 2.0) * size as f64 > y as f64 {
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

    fn add_generation_layer(&self, _generation_function: fn(i32, i32, i32, usize)) {
        todo!()
    }
}
