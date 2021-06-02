use crate::compute_renderer::uniforms::Uniforms;
use crate::compute_renderer::vertex::Vertex;
use crate::compute_renderer::wgpu_state::WgpuState;
use crate::renderer::shader_modules::shader_module_init;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroup, BindGroupLayout, BlendFactor, BlendOperation, BufferBinding, ComputePipeline,
    RenderPipeline, Sampler, SwapChainError, Texture, TextureView, TextureViewDimension,
};

pub struct Renderer {
    pub render_pipeline: wgpu::RenderPipeline,
    pub compute_pipeline: wgpu::ComputePipeline,
    pub texture_bind_group: BindGroup,
    pub compute_bind_group: BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub uniforms: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
}
impl Renderer {
    pub fn new(wgpu: &mut WgpuState) -> Renderer {
        let (vertex_buffer, index_buffer, num_indices, num_vertices) =
            Renderer::init_primitives(wgpu);
        let (
            texture_bind_group,
            texture_bind_group_layout,
            compute_bind_group,
            compute_bind_group_layout,
            uniforms,
            uniform_buffer,
        ) = Renderer::init_texture(wgpu);
        let render_pipeline = Renderer::init_pipeline(wgpu, texture_bind_group_layout);
        let compute_pipeline = Renderer::init_compute_pipeline(wgpu, compute_bind_group_layout);

        Renderer {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            num_vertices,
            texture_bind_group,
            compute_bind_group,
            compute_pipeline,
            uniforms,
            uniform_buffer,
        }
    }
    pub fn do_render_pass(&self, wgpu: &WgpuState) -> Result<(), SwapChainError> {
        let frame = wgpu.swap_chain.get_current_frame()?.output;

        wgpu.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );

        let mut encoder = wgpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &self.compute_bind_group, &[]);
            cpass.insert_debug_marker("compute screen pixels");
            cpass.dispatch(wgpu.size.width, wgpu.size.height, 1);
        }
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass world"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Default::default()),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        // submit will accept anything that implements IntoIter
        wgpu.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
    pub fn update(&mut self, viewer_pos: [f32; 3], time_diff: f64, viewing_dir: [f32; 3]) {
        self.uniforms
            .update_view_proj(viewer_pos, time_diff, viewing_dir);
    }
    pub fn init_compute_pipeline(
        wgpu: &mut WgpuState,
        compute_bind_group_layout: BindGroupLayout,
    ) -> ComputePipeline {
        let pipeline_layout = wgpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&compute_bind_group_layout],
                push_constant_ranges: &[],
            });
        let cs_module = shader_module_init("./shaders/compute.shader.comp.spv", &wgpu.device);
        let compute_pipeline =
            wgpu.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    module: &cs_module,
                    entry_point: "main",
                });
        return compute_pipeline;
    }
    pub fn init_pipeline(
        wgpu: &mut WgpuState,
        texture_bind_group_layout: BindGroupLayout,
    ) -> RenderPipeline {
        let vs_module = shader_module_init("./shaders/compute.shader.vert.spv", &wgpu.device);
        let fs_module = shader_module_init("./shaders/compute.shader.frag.spv", &wgpu.device);

        let render_pipeline_layout =
            wgpu.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&texture_bind_group_layout],
                    push_constant_ranges: &[],
                });
        let render_pipeline = wgpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vs_module,
                    entry_point: "main", // 1.
                    buffers: &[Vertex::desc()],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    clamp_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: Default::default(),
                fragment: Some(wgpu::FragmentState {
                    // 2.
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: WgpuState::get_sc_desc(wgpu.size).format,
                        write_mask: wgpu::ColorWrite::ALL,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: BlendFactor::SrcAlpha,
                                dst_factor: BlendFactor::Zero,
                                operation: BlendOperation::Add,
                            },
                            alpha: Default::default(),
                        }),
                    }],
                }),
            });
        return render_pipeline;
    }
    pub fn resized(&mut self, wgpu: &mut WgpuState) {
        let (bind_group, _, compute_bind_group, _, uniforms, uniform_buffer) =
            Renderer::init_texture(wgpu);
        self.texture_bind_group = bind_group;
        self.compute_bind_group = compute_bind_group;
        self.uniforms = uniforms;
        self.uniform_buffer = uniform_buffer;
    }
    pub fn init_primitives(wgpu: &mut WgpuState) -> (wgpu::Buffer, wgpu::Buffer, u32, u32) {
        let vertices = vec![
            Vertex {
                position: [-1.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];
        let indices = vec![0, 1, 2, 3, 0, 2];

        let vertices: &[Vertex] = vertices.as_slice();
        let indices: &[u16] = indices.as_slice();

        let vertex_buffer = wgpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });

        let index_buffer = wgpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsage::INDEX,
            });
        let num_indices = indices.len() as u32;
        let num_vertices = vertices.len() as u32;
        return (vertex_buffer, index_buffer, num_indices, num_vertices);
    }
    pub fn remake_texture(wgpu: &mut WgpuState) -> (Texture, TextureView) {
        let texture_size = wgpu::Extent3d {
            width: wgpu.size.width,
            height: wgpu.size.height,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = wgpu.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Uint,
            usage: wgpu::TextureUsage::STORAGE,
            label: Some("diffuse_texture"),
        });
        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("diffuse_texture_view"),
            format: Some(wgpu::TextureFormat::Rgba8Uint),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: Default::default(),
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });
        return (diffuse_texture, diffuse_texture_view);
    }
    pub fn init_uniforms(wgpu: &mut WgpuState) -> (wgpu::Buffer, Uniforms) {
        let uniforms = Uniforms::new();
        let uniform_buffer = wgpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            });
        return (uniform_buffer, uniforms);
    }
    pub fn init_texture(
        wgpu: &mut WgpuState,
    ) -> (
        BindGroup,
        BindGroupLayout,
        BindGroup,
        BindGroupLayout,
        Uniforms,
        wgpu::Buffer,
    ) {
        let (texture, diffuse_texture_view) = Renderer::remake_texture(wgpu);
        let texture_bind_group_layout =
            wgpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::ReadOnly,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            format: wgpu::TextureFormat::Rgba8Uint,
                        },
                        count: None,
                    }],
                    label: Some("texture_bind_group_layout"),
                });
        let diffuse_bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
            }],
            label: Some("diffuse_bind_group"),
        });
        let (uniform_buffer, uniforms) = Renderer::init_uniforms(wgpu);
        let compute_bind_group_layout =
            wgpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::WriteOnly,
                                view_dimension: TextureViewDimension::D2,
                                format: wgpu::TextureFormat::Rgba8Uint,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });
        let compute_bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        0: wgpu::BufferBinding {
                            buffer: &(uniform_buffer),
                            offset: 0,
                            size: None,
                        },
                    },
                },
            ],
        });

        return (
            diffuse_bind_group,
            texture_bind_group_layout,
            compute_bind_group,
            compute_bind_group_layout,
            uniforms,
            uniform_buffer,
        );
    }
}
