use crate::algorithms::noise_abstraction::Noise;
use noise::{MultiFractal, NoiseFn, Seedable};
use std::convert::TryInto;
use vox_core::constants::{BRICKMAPSIZE, BRICKSIZE};

pub struct NoiseSimd {
    seed: i32,
    octaves: u8,
    frequency: f32,
}

impl Noise for NoiseSimd {
    fn new(octaves: u32, seed: u32, frequency: f32) -> Self {
        Self {
            octaves: octaves as u8,
            seed: seed as i32,
            frequency,
        }
    }
    fn get(&self, x: f32, y: f32, z: f32, step_size: f32) -> [f32; 8] {
        let noise = simdnoise::NoiseBuilder::fbm_3d_offset(x, 1, y, 1, z, 8)
            .with_seed(self.seed as i32)
            .with_octaves(self.octaves as u8)
            .with_freq(self.frequency)
            .wrap();

        let noise = unsafe { simdnoise::avx2::get_3d_scaled_noise(&noise) };
        let result: [f32; 8] = noise.try_into().unwrap();
        result
    }
}
