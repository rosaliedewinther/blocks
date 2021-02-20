#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    _pos: [f32; 3],
    _color: [f32; 4],
    _normal: [f32; 3],
    _type: u32,
}

pub fn vertex(pos: [f32; 3], col: [u8; 4], nor: [f32; 3]) -> Vertex {
    Vertex {
        _pos: [pos[0], pos[1], pos[2]],
        _color: [col[0] as f32, col[1] as f32, col[2] as f32, col[3] as f32],
        _normal: [nor[0], nor[1], nor[2]],
        _type: 0,
    }
}
pub fn vertex_typed(pos: [f32; 3], col: [u8; 4], nor: [f32; 3], block_type: u32) -> Vertex {
    Vertex {
        _pos: [pos[0], pos[1], pos[2]],
        _color: [col[0] as f32, col[1] as f32, col[2] as f32, col[3] as f32],
        _normal: [nor[0], nor[1], nor[2]],
        _type: block_type,
    }
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex, // 2.
            attributes: &[
                // 3.
                wgpu::VertexAttribute {
                    offset: 0,                          // 4.
                    shader_location: 0,                 // 5.
                    format: wgpu::VertexFormat::Float3, // 6.
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>()
                        + std::mem::size_of::<[f32; 4]>()
                        + std::mem::size_of::<[f32; 3]>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint,
                },
            ],
        }
    }
}
