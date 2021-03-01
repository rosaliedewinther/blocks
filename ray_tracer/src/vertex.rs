#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
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
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
    }
}
