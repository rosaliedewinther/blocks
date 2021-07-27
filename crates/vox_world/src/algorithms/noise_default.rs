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
    fn get(&self, x: f32, y: f32, z: f32, step_size: f32) -> [f32; 8] {
        let x = x as f64;
        let y = y as f64;
        let z = z as f64;
        let step_size = step_size as f64;
        [
            self.noise.get([x, y, z]) as f32,
            self.noise.get([x, y, z + step_size * 1.0]) as f32,
            self.noise.get([x, y, z + step_size * 2.0]) as f32,
            self.noise.get([x, y, z + step_size * 3.0]) as f32,
            self.noise.get([x, y, z + step_size * 4.0]) as f32,
            self.noise.get([x, y, z + step_size * 5.0]) as f32,
            self.noise.get([x, y, z + step_size * 6.0]) as f32,
            self.noise.get([x, y, z + step_size * 7.0]) as f32,
        ]
    }
}
