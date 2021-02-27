use crate::constants::CHUNKSIZE;
use crate::renderer::vertex::Vertex;
use crate::renderer::wgpu::WgpuState;
use crate::world_gen::chunk::ChunkData;
use futures::executor::block_on;
use memcmp::Memcmp;
use std::borrow::Cow;
use std::convert::TryInto;
use std::time::Instant;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, Buffer, ComputePipeline, Device, Instance, Queue, SwapChainTexture};
use winit::window::Window;

pub struct Compute {
    bind_group: BindGroup,
    cpu_buffer: Buffer,
    gpu_buffer_in: Buffer,
    gpu_buffer_out: Buffer,
    pipeline: ComputePipeline,
}

impl Compute {
    pub fn new(device: &Device, queue: &Queue, window: &Window) -> Compute {
        block_on(async {
            let cs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    "../shaders/shader.wgsl"
                ))),
                flags: wgpu::ShaderFlags::VALIDATION,
            });
            let slice_size = std::mem::size_of::<ChunkData>();
            let chunk_size = slice_size as wgpu::BufferAddress;
            let vertex_array_size = (std::mem::size_of::<ChunkData>()
                * std::mem::size_of::<Vertex>())
                as wgpu::BufferAddress;

            // Instantiates buffer without data.
            // `usage` of buffer specifies how it can be used:
            //   `BufferUsage::MAP_READ` allows it to be read (outside the shader).
            //   `BufferUsage::COPY_DST` allows it to be the destination of the copy.
            let cpu_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: vertex_array_size,
                usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            });

            // Instantiates buffer with data (`numbers`).
            // Usage allowing the buffer to be:
            //   A storage buffer (can be bound within a bind group and thus available to a shader).
            //   The destination of a copy.
            //   The source of a copy.
            let mut gpu_buffer_in = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Storage Buffer"),
                size: chunk_size,
                usage: wgpu::BufferUsage::STORAGE
                    | wgpu::BufferUsage::COPY_DST
                    | wgpu::BufferUsage::COPY_SRC,
                mapped_at_creation: false,
            });
            let mut gpu_buffer_out = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Storage Buffer"),
                size: vertex_array_size,
                usage: wgpu::BufferUsage::STORAGE
                    | wgpu::BufferUsage::COPY_DST
                    | wgpu::BufferUsage::COPY_SRC,
                mapped_at_creation: false,
            });

            // A bind group defines how buffers are accessed by shaders.
            // It is to WebGPU what a descriptor set is to Vulkan.
            // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).

            // Here we specifiy the layout of the bind group.
            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,                             // The location
                            visibility: wgpu::ShaderStage::COMPUTE, // Which shader type in the pipeline this buffer is available to.
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage {
                                    // Specifies if the buffer can only be read within the shader
                                    read_only: true,
                                },
                                has_dynamic_offset: false,
                                min_binding_size: wgpu::BufferSize::new(4),
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,                             // The location
                            visibility: wgpu::ShaderStage::COMPUTE, // Which shader type in the pipeline this buffer is available to.
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage {
                                    // Specifies if the buffer can only be read within the shader
                                    read_only: false,
                                },
                                has_dynamic_offset: false,
                                min_binding_size: wgpu::BufferSize::new(4),
                            },
                            count: None,
                        },
                    ],
                });

            // Instantiates the bind group, once again specifying the binding of buffers.
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: gpu_buffer_in.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: gpu_buffer_out.as_entire_binding(),
                    },
                ],
            });

            // A pipeline specifices the operation of a shader

            // Here we specifiy the layout of the pipeline.
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

            let sc_desc = WgpuState::get_sc_desc(window.inner_size());
            // Instantiates the pipeline.
            let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                module: &cs_module,
                entry_point: "main",
            });

            return Compute {
                bind_group,
                cpu_buffer,
                gpu_buffer_in,
                gpu_buffer_out,
                pipeline,
            };
        })
    }
    pub fn compute_pass(&mut self, device: &Device, queue: &Queue, main: &[u8]) {
        let timer = Instant::now();
        const SIZE: u64 = (std::mem::size_of::<ChunkData>() * std::mem::size_of::<Vertex>())
            as wgpu::BufferAddress;
        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        queue.write_buffer(&self.gpu_buffer_in, 0, main);

        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&self.pipeline);
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.insert_debug_marker("compute collatz iterations");
            cpass.dispatch((CHUNKSIZE * CHUNKSIZE * CHUNKSIZE) as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }
        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_buffer_to_buffer(&self.gpu_buffer_out, 0, &self.cpu_buffer, 0, SIZE);

        // Submits command encoder for processing
        queue.submit(Some(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = self.cpu_buffer.slice(..);
        // Gets the future representing when `staging_buffer` can be read from
        let buffer_future = buffer_slice.map_async(wgpu::MapMode::Read);
        println!("{}", timer.elapsed().as_secs_f64());
        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        device.poll(wgpu::Maintain::Wait);

        // Awaits until `buffer_future` can be read from
        block_on(async {
            if let Ok(()) = buffer_future.await {
                // Gets contents of buffer
                let data = buffer_slice.get_mapped_range();
                println!("{}", timer.elapsed().as_secs_f64());
                // Since contents are got in bytes, this converts these bytes back to u32
                let result: Vec<Vertex> = data
                    .chunks_exact(std::mem::size_of::<Vertex>())
                    .map(|b| *bytemuck::from_bytes::<Vertex>(b))
                    .filter(|v| bytemuck::bytes_of(v).memcmp(&[0u8; 32]))
                    .collect();
                // With the current interface, we have to make sure all mapped views are
                // dropped before we unmap the buffer.
                drop(data);

                self.cpu_buffer.unmap(); // Unmaps buffer from memory
                                         // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                         //   delete myPointer;
                                         //   myPointer = NULL;
                                         // It effectively frees the memory

            // Returns data from buffer
            //println!("{:?}", result);
            } else {
                panic!("failed to run compute on gpu!")
            }
            println!("{}", timer.elapsed().as_secs_f64());
        });
    }
}
