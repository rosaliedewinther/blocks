use crate::uniforms::Uniforms;
use crate::vertex::Vertex;
use wgpu::util::DeviceExt;
use wgpu::{
    BlendFactor, BlendOperation, Device, Queue, RenderPass, SwapChainDescriptor, VertexState,
};

pub struct WgpuPipeline {
    pub uniform_buffer: wgpu::Buffer,
    pub uniforms: Uniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl WgpuPipeline {
    pub fn new(device: &Device, sc_desc: &SwapChainDescriptor) -> WgpuPipeline {
        let uniforms = Uniforms::new();

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        min_binding_size: None,
                        has_dynamic_offset: false,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &(uniform_buffer),
                    offset: 0,
                    size: None,
                },
            }],
            label: Some("uniform_bind_group"),
        });

        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("./shaders/main.shader.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("./shaders/main.shader.frag.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main", // 1.
                buffers: &[Vertex::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: Some(wgpu::IndexFormat::Uint32),
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                // 2.
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState {
                        src_factor: BlendFactor::SrcAlpha,
                        dst_factor: BlendFactor::OneMinusSrcAlpha,
                        operation: BlendOperation::Add,
                    },
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
        });

        return WgpuPipeline {
            uniforms,
            uniform_bind_group,
            uniform_buffer,
            render_pipeline,
        };
    }
    pub fn setup_render_pass<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
    }
    pub fn set_uniform_buffer(&self, queue: &Queue, uniforms: Uniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
}
