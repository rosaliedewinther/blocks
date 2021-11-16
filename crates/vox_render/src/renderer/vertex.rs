#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub _pos: [f32; 3],
    pub _material: u32,
    pub _normal: u32,
}

pub fn vertex(pos: [f32; 3], material: u32, nor: u32) -> Vertex {
    Vertex {
        _pos: [pos[0], pos[1], pos[2]],
        _material: material,
        _normal: nor,
    }
}
pub fn vertex_typed(pos: [f32; 3], material: u32, nor: u32) -> Vertex {
    Vertex {
        _pos: [pos[0], pos[1], pos[2]],
        _material: material,
        _normal: nor,
    }
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex, // 2.
            attributes: &[
                // 3.
                wgpu::VertexAttribute {
                    offset: 0,                             // 4.
                    shader_location: 0,                    // 5.
                    format: wgpu::VertexFormat::Float32x3, // 6.
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32;3]>() + std::mem::size_of::<u32>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}
