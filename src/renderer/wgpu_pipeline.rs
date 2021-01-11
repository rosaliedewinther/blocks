use crate::block::{Block, BlockSides, BlockType};
use crate::positions::{GlobalBlockPos, MetaChunkPos};
use crate::renderer::depth_texture::DepthTexture;
use crate::renderer::uniforms::Uniforms;
use crate::renderer::vertex::Vertex;
use crate::world_gen::meta_chunk::MetaChunk;
use wgpu::util::DeviceExt;
use wgpu::{CommandEncoder, Device, Queue, RenderPass, SwapChainDescriptor, SwapChainTexture};

pub struct WgpuPipeline {
    pub uniform_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub uniforms: Uniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub num_vertices: u32,
    pub render_pipeline: wgpu::RenderPipeline,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl WgpuPipeline {
    pub fn new(device: &Device, sc_desc: &SwapChainDescriptor, pos: i32) -> WgpuPipeline {
        let mut uniforms = Uniforms::new();

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
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });

        let vs_module =
            device.create_shader_module(wgpu::include_spirv!("../shaders/main.shader.vert.spv"));
        let fs_module =
            device.create_shader_module(wgpu::include_spirv!("../shaders/main.shader.frag.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main", // 1.
            },
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint32,
                vertex_buffers: &[Vertex::desc()],
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                // 2.
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList, // 1.
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilStateDescriptor::default(), // 2.
            }), // 2.
            sample_count: 1,                                           // 5.
            sample_mask: !0,                                           // 6.
            alpha_to_coverage_enabled: false,                          // 7.
        });

        let chunk = MetaChunk::load_or_gen(MetaChunkPos { x: pos, z: 0 }, 1, false);

        let (vertices, indices) = chunk.generate_vertex_buffers();

        let vertices: &[Vertex] = vertices.as_slice();
        let indices: &[u32] = indices.as_slice();

        println!("ind: {:?}", indices);
        println!("vert size: {:#?}", vertices.len());
        println!("ind size {:#?}", indices.len());

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let num_indices = indices.len() as u32;
        let num_vertices = vertices.len() as u32;
        return WgpuPipeline {
            uniforms,
            uniform_bind_group,
            uniform_buffer,
            vertex_buffer,
            num_vertices,
            render_pipeline,
            index_buffer,
            num_indices,
        };
    }
    pub fn set_vertices(&mut self, queue: &Queue, vertices: &[Vertex]) {
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertices));
        self.num_vertices = vertices.len() as u32;
    }
    pub fn do_render_pass<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..));
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
    pub fn set_uniform_buffer(&self, queue: &Queue, uniforms: Uniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
}
