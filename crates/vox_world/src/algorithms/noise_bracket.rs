use crate::algorithms::noise_abstraction::Noise;
use bracket_noise::prelude::{CellularDistanceFunction, CellularReturnType, FastNoise, NoiseType};

pub struct NoiseBracket {
    noise: bracket_noise::prelude::FastNoise,
}

impl Noise for NoiseBracket {
    fn new(octaves: u32, seed: u32, frequency: f32) -> Self {
        let mut noise = FastNoise::seeded(seed as u64);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_frequency(frequency);
        noise.set_fractal_octaves(octaves as i32);
        noise.set_cellular_distance_function(CellularDistanceFunction::Euclidean);
        noise.set_cellular_return_type(CellularReturnType::CellValue);

        Self { noise }
    }
    fn get(&self, x: f32, y: f32, z: f32) -> f32 {
        self.noise.get_noise3d(x, y, z) as f32
    }
}
