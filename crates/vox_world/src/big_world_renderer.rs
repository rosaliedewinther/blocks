use crate::blocks::block::BlockId;
use crate::player::Player;
use log::warn;
use std::collections::HashMap;
use vox_core::constants::WORLD_SIZE;
use vox_render::compute_renderer::renderpassable::RenderPassable;
use vox_render::compute_renderer::shader_modules::shader_module_init;
use vox_render::compute_renderer::uniforms::Uniforms;
use vox_render::compute_renderer::wgpu_state::WgpuState;
use wgpu::util::DeviceExt;
use wgpu::CommandEncoder;
use winit::window::Window;

pub struct BigWorldRenderer {
    rendering_pipeline: wgpu::ComputePipeline,
    rendering_bind_group: wgpu::BindGroup,
    sdf_pipeline: wgpu::ComputePipeline,
    sdf_bind_group: wgpu::BindGroup,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    world_buffer: wgpu::Buffer,
    sdf_buffer: wgpu::Buffer,
}

impl BigWorldRenderer {
    pub fn new(wgpu_state: &WgpuState, texture_to_draw_to: &wgpu::TextureView) -> BigWorldRenderer {
        let (uniform_buffer, uniforms) = BigWorldRenderer::init_uniforms(wgpu_state);
        let world_buffer = BigWorldRenderer::init_world_buffer(wgpu_state);
        let sdf_buffer = BigWorldRenderer::init_sdf_buffer(wgpu_state);
        let (rendering_pipeline, rendering_bind_group) = BigWorldRenderer::init_rendering_pipeline(
            wgpu_state,
            texture_to_draw_to,
            &uniform_buffer,
            &world_buffer,
            &sdf_buffer,
        );
        let (sdf_pipeline, sdf_bind_group) =
            BigWorldRenderer::init_sdf_pipeline(wgpu_state, &world_buffer, &sdf_buffer);
        BigWorldRenderer {
            rendering_pipeline,
            rendering_bind_group,
            sdf_pipeline,
            sdf_bind_group,
            uniforms,
            uniform_buffer,
            world_buffer,
            sdf_buffer,
        }
    }
    fn init_uniforms(wgpu_state: &WgpuState) -> (wgpu::Buffer, Uniforms) {
        let uniforms = Uniforms::new();
        let uniform_buffer =
            wgpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Uniform Buffer"),
                    contents: bytemuck::cast_slice(&[uniforms]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        return (uniform_buffer, uniforms);
    }
    fn init_world_buffer(wgpu_state: &WgpuState) -> wgpu::Buffer {
        let buffer_descriptor = wgpu::BufferDescriptor {
            label: Some("world buffer"),
            size: (WORLD_SIZE.pow(3) * std::mem::size_of::<u8>()) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        };
        let world_buffer = wgpu_state.device.create_buffer(&buffer_descriptor);
        warn!(
            "initialized world with size of {} bytes",
            WORLD_SIZE.pow(3) * std::mem::size_of::<u8>()
        );
        return world_buffer;
    }
    fn init_sdf_buffer(wgpu_state: &WgpuState) -> wgpu::Buffer {
        let buffer_descriptor = wgpu::BufferDescriptor {
            label: Some("sdf buffer"),
            size: (WORLD_SIZE.pow(3) * std::mem::size_of::<u8>()) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        };
        let sdf_buffer = wgpu_state.device.create_buffer(&buffer_descriptor);
        warn!(
            "initialized sdf with size of {} bytes",
            WORLD_SIZE.pow(3) * std::mem::size_of::<u8>()
        );
        return sdf_buffer;
    }
    /*pub fn rebuild_sdf(
        &self,
        brick_index: u32,
        data: &[u8; BRICKSIZE.pow(3)],
        wgpu_state: &WgpuState,
    ) {
        let mut encoder = wgpu_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        wgpu_state.queue.submit(std::iter::once(encoder.finish()));
    }*/
    pub fn upload_world(&self, world: &[BlockId], wgpu_state: &WgpuState) {
        let uploading_buffer =
            wgpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("world uploading Buffer"),
                    contents: bytemuck::cast_slice(world),
                    usage: wgpu::BufferUsages::COPY_SRC,
                });
        let mut encoder = wgpu_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(
            &uploading_buffer,
            0,
            &self.world_buffer,
            0,
            (WORLD_SIZE.pow(3) * std::mem::size_of::<u8>()) as u64,
        );
        wgpu_state.queue.submit(std::iter::once(encoder.finish()));
    }
    fn init_rendering_pipeline(
        wgpu_state: &WgpuState,
        diffuse_texture_view: &wgpu::TextureView,
        uniform_buffer: &wgpu::Buffer,
        world_buffer: &wgpu::Buffer,
        sdf_buffer: &wgpu::Buffer,
    ) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
        let compute_bind_group_layout =
            wgpu_state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::WriteOnly,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                format: wgpu::TextureFormat::Rgba8Uint,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });
        let compute_bind_group = wgpu_state
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
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
                                buffer: &(world_buffer),
                                offset: 0,
                                size: None,
                            },
                        },
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Buffer {
                            0: wgpu::BufferBinding {
                                buffer: &(sdf_buffer),
                                offset: 0,
                                size: None,
                            },
                        },
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
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
        let pipeline_layout =
            wgpu_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&compute_bind_group_layout],
                    push_constant_ranges: &[],
                });
        let cs_module = shader_module_init("./shaders/compute.comp.spv", &wgpu_state.device);
        let compute_pipeline =
            wgpu_state
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    module: &cs_module,
                    entry_point: "main",
                });
        return (compute_pipeline, compute_bind_group);
    }
    fn init_sdf_pipeline(
        wgpu_state: &WgpuState,
        world_buffer: &wgpu::Buffer,
        sdf_buffer: &wgpu::Buffer,
    ) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
        let sdf_bind_group_layout =
            wgpu_state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });
        let sdf_bind_group = wgpu_state
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &sdf_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            0: wgpu::BufferBinding {
                                buffer: &(world_buffer),
                                offset: 0,
                                size: None,
                            },
                        },
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer {
                            0: wgpu::BufferBinding {
                                buffer: &(sdf_buffer),
                                offset: 0,
                                size: None,
                            },
                        },
                    },
                ],
            });
        let sdf_pipeline_layout =
            wgpu_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&sdf_bind_group_layout],
                    push_constant_ranges: &[],
                });
        let cs_module =
            shader_module_init("./shaders/sdf_calculation.comp.spv", &wgpu_state.device);
        let sdf_pipeline =
            wgpu_state
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: Some(&sdf_pipeline_layout),
                    module: &cs_module,
                    entry_point: "main",
                });
        return (sdf_pipeline, sdf_bind_group);
    }
    pub fn update_all_buffers(&mut self, wgpu_state: &WgpuState, player: &Player, time_diff: f64) {
        let location = [
            player.position.x as f32,
            player.position.y as f32,
            player.position.z as f32,
        ];
        let direction = [
            player.direction[0] as f32,
            player.direction[1] as f32,
            player.direction[2] as f32,
        ];
        self.uniforms.update_view_proj(
            location,
            time_diff,
            direction,
            [wgpu_state.size.width, wgpu_state.size.height],
        );
        wgpu_state.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }
}

impl RenderPassable for BigWorldRenderer {
    fn do_render_pass<'a>(
        &'a mut self,
        window: &Window,
        encoder: &mut CommandEncoder,
        wgpu_state: &WgpuState,
        frame: &wgpu::TextureView,
    ) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&self.rendering_pipeline);
        cpass.set_bind_group(0, &self.rendering_bind_group, &[]);
        cpass.insert_debug_marker("compute screen pixels");
        cpass.dispatch(
            (wgpu_state.size.width as f32 / 8.0).ceil() as u32,
            (wgpu_state.size.height as f32 / 8.0).ceil() as u32,
            1,
        );
    }
}
