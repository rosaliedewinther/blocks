use crate::block::{Block, BlockSides, BlockType};
use crate::player::Player;
use crate::positions::{GlobalBlockPos, ObjectPos};
use crate::renderer::depth_texture::DepthTexture;
use crate::renderer::uniforms::Uniforms;
use crate::renderer::vertex::{vertex, Vertex};
use crate::renderer::wgpu_pipeline::WgpuPipeline;
use futures::executor::block_on;
use imgui_wgpu::Texture;
use std::f32::consts::PI;
use std::time::Instant;
use wgpu::util::DeviceExt;
use wgpu::TextureFormat;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,

    //index_buffer: wgpu::Buffer,
    //num_indices: u32,
    pipeline: WgpuPipeline,
    pipeline2: WgpuPipeline,
    depth_texture: DepthTexture,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let mut pipeline = WgpuPipeline::new(&device, &sc_desc, 0);
        let mut pipeline2 = WgpuPipeline::new(&device, &sc_desc, 1);

        //pipeline.set_vertices(&queue, vertices);

        /*let indices: &[u16] = &[0, 1, 2, 3, 1, 2];
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let num_indices = indices.len() as u32;*/
        let depth_texture = DepthTexture::create_depth_texture(&device, &sc_desc, "depth_texture");

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            //index_buffer,
            //num_indices,
            pipeline,
            pipeline2,
            depth_texture,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.6,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            //self.pipeline.do_render_pass(&mut render_pass);
            //self.pipeline2.do_render_pass(&mut render_pass);

            render_pass.set_pipeline(&self.pipeline.render_pipeline);
            render_pass.set_bind_group(0, &self.pipeline.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.pipeline.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.pipeline.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.pipeline.num_indices, 0, 0..1);
            render_pass.set_vertex_buffer(0, self.pipeline2.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.pipeline2.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.pipeline2.num_indices, 0, 0..1);
        }
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

pub fn gen_perspective_mat(size: (u32, u32)) -> [[f32; 4]; 4] {
    let (width, height) = size;
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = PI / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 1.0],
    ]
}

pub fn start_main_loop() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state = block_on(State::new(&window));
    let mut player = Player::new();
    player.position = ObjectPos {
        x: 0f32,
        y: 0f32,
        z: 0f32,
    };

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        state.resize(**new_inner_size);
                    }

                    _ => {}
                }
            }
        }
        Event::RedrawRequested(_) => {
            player.handle_input(&(0.01 as f32));
            player.update(&(0.01 as f32));
            state
                .pipeline
                .uniforms
                .update_view_proj(&player, (state.size.width, state.size.height));
            state
                .pipeline
                .set_uniform_buffer(&state.queue, state.pipeline.uniforms);
            state.update();
            match state.render() {
                Ok(_) => {}
                // Recreate the swap_chain if lost
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    });
}
