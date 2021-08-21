use crate::algorithms::noise_abstraction::Noise;
use noise::{MultiFractal, NoiseFn, Seedable};

pub struct NoiseDefault {
    noise: noise::Fbm,
}

impl Noise for NoiseDefault {
    fn new(octaves: u32, seed: u32, frequency: f32) -> Self {
        Self {
            noise: noise::Fbm::new()
                .set_seed(seed)
                .set_octaves(octaves as usize)
                .set_frequency(frequency as f64),
        }
    }
    fn get(&self, x: f32, y: f32, z: f32) -> f32 {
        let x = x as f64;
        let y = y as f64;
        let z = z as f64;

        self.noise.get([x, y, z]) as f32
    }
}
