pub trait Noise {
    fn new(octaves: u32, seed: u32, frequency: f32) -> Self;
    fn get(&self, x: f32, y: f32, z: f32) -> f32;
}
