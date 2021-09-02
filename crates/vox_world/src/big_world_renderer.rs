use crate::blocks::block::BlockId;
use crate::player::Player;
use log::warn;
use std::num::NonZeroU32;
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
    world_texture: wgpu::Texture,
    world_textureview: wgpu::TextureView,
    sdf_texture: wgpu::Texture,
    sdf_textureview: wgpu::TextureView,
}

impl BigWorldRenderer {
    pub fn new(wgpu_state: &WgpuState, texture_to_draw_to: &wgpu::TextureView) -> BigWorldRenderer {
        let (uniform_buffer, uniforms) = BigWorldRenderer::init_uniforms(wgpu_state);
        let (world_texture, world_textureview) = BigWorldRenderer::init_world_buffer(wgpu_state);
        let (sdf_texture, sdf_textureview) = BigWorldRenderer::init_sdf_buffer(wgpu_state);
        let (sdf_pipeline, sdf_bind_group) =
            BigWorldRenderer::init_sdf_pipeline(wgpu_state, &world_textureview, &sdf_textureview);
        let (rendering_pipeline, rendering_bind_group) = BigWorldRenderer::init_rendering_pipeline(
            wgpu_state,
            texture_to_draw_to,
            &uniform_buffer,
            &world_textureview,
            &sdf_textureview,
        );

        BigWorldRenderer {
            rendering_pipeline,
            rendering_bind_group,
            sdf_pipeline,
            sdf_bind_group,
            uniforms,
            uniform_buffer,
            world_texture,
            world_textureview,
            sdf_texture,
            sdf_textureview,
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
    fn init_world_buffer(wgpu_state: &WgpuState) -> (wgpu::Texture, wgpu::TextureView) {
        let texture_size = wgpu::Extent3d {
            width: WORLD_SIZE as u32,
            height: WORLD_SIZE as u32,
            depth_or_array_layers: WORLD_SIZE as u32,
        };
        let texture = wgpu_state.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("sdf_texture"),
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("sdf_texture_view"),
            format: Some(wgpu::TextureFormat::R8Uint),
            dimension: Some(wgpu::TextureViewDimension::D3),
            aspect: Default::default(),
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });
        warn!("initialized world texture with size of {}^3", WORLD_SIZE);
        return (texture, texture_view);
    }
    fn init_sdf_buffer(wgpu_state: &WgpuState) -> (wgpu::Texture, wgpu::TextureView) {
        let texture_size = wgpu::Extent3d {
            width: WORLD_SIZE as u32,
            height: WORLD_SIZE as u32,
            depth_or_array_layers: WORLD_SIZE as u32,
        };
        let texture = wgpu_state.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::STORAGE_BINDING,
            label: Some("sdf_texture"),
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("sdf_texture_view"),
            format: Some(wgpu::TextureFormat::R8Uint),
            dimension: Some(wgpu::TextureViewDimension::D3),
            aspect: Default::default(),
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });
        warn!("initialized sdf texture with size of {}^3", WORLD_SIZE);
        return (texture, texture_view);
    }
    pub fn rebuild_sdf(&self, wgpu_state: &WgpuState) {
        let mut encoder = wgpu_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&self.sdf_pipeline);
            cpass.set_bind_group(0, &self.sdf_bind_group, &[]);
            cpass.insert_debug_marker("sdf calculation");
            cpass.dispatch(
                (WORLD_SIZE as f32 / 8.0).ceil() as u32,
                (WORLD_SIZE as f32 / 8.0).ceil() as u32,
                1,
            );
        }
        wgpu_state.queue.submit(std::iter::once(encoder.finish()));
    }
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
        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &uploading_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(NonZeroU32::new(WORLD_SIZE as u32).unwrap()),
                    rows_per_image: Some(NonZeroU32::new(WORLD_SIZE as u32).unwrap()),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &self.world_texture,
                mip_level: 0,
                origin: Default::default(),
                aspect: Default::default(),
            },
            wgpu::Extent3d {
                width: WORLD_SIZE as u32,
                height: WORLD_SIZE as u32,
                depth_or_array_layers: WORLD_SIZE as u32,
            },
        );
        wgpu_state.queue.submit(std::iter::once(encoder.finish()));
    }
    fn init_rendering_pipeline(
        wgpu_state: &WgpuState,
        diffuse_texture_view: &wgpu::TextureView,
        uniform_buffer: &wgpu::Buffer,
        world_texture: &wgpu::TextureView,
        sdf_texture: &wgpu::TextureView,
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
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::ReadOnly,
                                view_dimension: wgpu::TextureViewDimension::D3,
                                format: wgpu::TextureFormat::R8Uint,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::ReadOnly,
                                view_dimension: wgpu::TextureViewDimension::D3,
                                format: wgpu::TextureFormat::R8Uint,
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
                        resource: wgpu::BindingResource::TextureView(&world_texture),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&sdf_texture),
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
        println!("created rendering pipeline");
        return (compute_pipeline, compute_bind_group);
    }
    fn init_sdf_pipeline(
        wgpu_state: &WgpuState,
        world_texture: &wgpu::TextureView,
        sdf_texture: &wgpu::TextureView,
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
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::ReadOnly,
                                view_dimension: wgpu::TextureViewDimension::D3,
                                format: wgpu::TextureFormat::R8Uint,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::ReadWrite,
                                view_dimension: wgpu::TextureViewDimension::D3,
                                format: wgpu::TextureFormat::R8Uint,
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
                        resource: wgpu::BindingResource::TextureView(&world_texture),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&sdf_texture),
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
        println!("created sdf pipeline");
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
            player.view_type,
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
        _window: &Window,
        encoder: &mut CommandEncoder,
        wgpu_state: &WgpuState,
        _frame: &wgpu::TextureView,
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
