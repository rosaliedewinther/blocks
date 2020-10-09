#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

pub type Color = [f32; 4];

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}
