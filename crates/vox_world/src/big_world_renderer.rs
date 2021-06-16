use crate::player::Player;
use std::collections::HashMap;
use vox_core::constants::{BRICKMAPSIZE, BRICKSIZE};
use vox_render::compute_renderer::renderpassable::RenderPassable;
use vox_render::compute_renderer::shader_modules::shader_module_init;
use vox_render::compute_renderer::uniforms::Uniforms;
use vox_render::compute_renderer::wgpu_state::WgpuState;
use wgpu::util::DeviceExt;
use wgpu::{CommandEncoder, SwapChainTexture};
use winit::window::Window;

pub struct BigWorldRenderer {
    compute_pipeline: wgpu::ComputePipeline,
    compute_bind_group: wgpu::BindGroup,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    brick_map_buffer: wgpu::Buffer,
    bricks_buffer: wgpu::Buffer,
}

impl BigWorldRenderer {
    pub fn new(wgpu_state: &WgpuState, texture_to_draw_to: &wgpu::TextureView) -> BigWorldRenderer {
        let (uniform_buffer, uniforms) = BigWorldRenderer::init_uniforms(wgpu_state);
        let brick_map_buffer =
            BigWorldRenderer::init_brickmaps(wgpu_state, BRICKMAPSIZE.pow(3) as u32 * 27);
        let bricks_buffer = BigWorldRenderer::init_bricks(
            wgpu_state,
            (27 * (BRICKMAPSIZE * BRICKSIZE).pow(3)) as u32,
        );
        let (compute_bind_group_layout, compute_bind_group) =
            BigWorldRenderer::init_compute_bind_group(
                wgpu_state,
                texture_to_draw_to,
                &uniform_buffer,
                &brick_map_buffer,
                &bricks_buffer,
            );
        let compute_pipeline =
            BigWorldRenderer::init_compute_pipeline(wgpu_state, compute_bind_group_layout);
        BigWorldRenderer {
            compute_pipeline,
            compute_bind_group,
            uniforms,
            uniform_buffer,
            brick_map_buffer,
            bricks_buffer,
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
                    usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                });
        return (uniform_buffer, uniforms);
    }
    fn init_compute_pipeline(
        wgpu_state: &WgpuState,
        compute_bind_group_layout: wgpu::BindGroupLayout,
    ) -> wgpu::ComputePipeline {
        let pipeline_layout =
            wgpu_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&compute_bind_group_layout],
                    push_constant_ranges: &[],
                });
        let cs_module = shader_module_init("./shaders/compute.shader.comp.spv", &wgpu_state.device);
        let compute_pipeline =
            wgpu_state
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    module: &cs_module,
                    entry_point: "main",
                });
        return compute_pipeline;
    }
    fn init_brickmaps(wgpu_state: &WgpuState, amount_of_bricks: u32) -> wgpu::Buffer {
        let buffer_descriptor = wgpu::BufferDescriptor {
            label: Some("brickmap buffer"),
            size: amount_of_bricks as u64 * std::mem::size_of::<u32>() as u64,
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::STORAGE,
            mapped_at_creation: false,
        };
        let brick_map_buffer = wgpu_state.device.create_buffer(&buffer_descriptor);

        return brick_map_buffer;
    }
    fn init_bricks(wgpu_state: &WgpuState, max_amount_of_bricks: u32) -> wgpu::Buffer {
        let bricks_buffer =
            wgpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("brick Buffer"),
                    contents: bytemuck::cast_slice(
                        vec![255u8; max_amount_of_bricks as usize].as_slice(),
                    ),
                    usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::STORAGE,
                });
        /*let buffer_descriptor = wgpu::BufferDescriptor {
            label: Some("bricks buffer"),
            size: max_amount_of_bricks as u64 * std::mem::size_of::<u8>() as u64,
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::STORAGE,
            mapped_at_creation: false,
        };
        let bricks_buffer = wgpu_state.device.create_buffer(&buffer_descriptor);*/

        return bricks_buffer;
    }
    pub fn set_brick(
        &self,
        brick_index: u32,
        data: &[u8; BRICKSIZE.pow(3)],
        wgpu_state: &WgpuState,
    ) {
        let uploading_buffer =
            wgpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("brick uploading Buffer"),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsage::COPY_SRC,
                });
        let mut encoder = wgpu_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(
            &uploading_buffer,
            0,
            &self.bricks_buffer,
            (std::mem::size_of::<u8>() * brick_index as usize * BRICKSIZE.pow(3)) as u64,
            (std::mem::size_of::<u8>() * BRICKSIZE.pow(3)) as u64,
        );
        wgpu_state.queue.submit(std::iter::once(encoder.finish()));
    }
    pub fn set_brickmap(
        &self,
        brickmap_index: u32,
        data: &Box<[u32; BRICKMAPSIZE.pow(3) * 27]>,
        wgpu_state: &WgpuState,
    ) {
        let uploading_buffer =
            wgpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("brickmap uploading Buffer"),
                    contents: bytemuck::cast_slice(data.as_ref()),
                    usage: wgpu::BufferUsage::COPY_SRC,
                });
        let mut encoder = wgpu_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(
            &uploading_buffer,
            0,
            &self.brick_map_buffer,
            (std::mem::size_of::<u32>() * brickmap_index as usize * BRICKMAPSIZE.pow(3)) as u64,
            (std::mem::size_of::<u32>() * BRICKMAPSIZE.pow(3)) as u64,
        );
        wgpu_state.queue.submit(std::iter::once(encoder.finish()));
    }
    fn init_compute_bind_group(
        wgpu_state: &WgpuState,
        diffuse_texture_view: &wgpu::TextureView,
        uniform_buffer: &wgpu::Buffer,
        brick_map_buffer: &wgpu::Buffer,
        bricks_buffer: &wgpu::Buffer,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let compute_bind_group_layout =
            wgpu_state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::WriteOnly,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                format: wgpu::TextureFormat::Rgba8Uint,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStage::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
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
                                buffer: &(brick_map_buffer),
                                offset: 0,
                                size: None,
                            },
                        },
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Buffer {
                            0: wgpu::BufferBinding {
                                buffer: &(bricks_buffer),
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
        return (compute_bind_group_layout, compute_bind_group);
    }
    pub fn update_all_buffers(&mut self, wgpu_state: &WgpuState, player: &Player, time_diff: f64) {
        let location = [player.position.x, player.position.y, player.position.z];
        let direction = [
            player.direction[0],
            player.direction[1],
            player.direction[2],
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
        frame: &SwapChainTexture,
    ) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&self.compute_pipeline);
        cpass.set_bind_group(0, &self.compute_bind_group, &[]);
        cpass.insert_debug_marker("compute screen pixels");
        cpass.dispatch(
            (wgpu_state.size.width as f32 / 8.0).ceil() as u32,
            (wgpu_state.size.height as f32 / 8.0).ceil() as u32,
            1,
        );
    }
}
