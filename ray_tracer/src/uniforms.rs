#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    position: [f32; 3],
    time: f32,
    direction: [f32; 3],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            time: 0.0,
            direction: [0.0, 0.0, -1.0],
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }
}
