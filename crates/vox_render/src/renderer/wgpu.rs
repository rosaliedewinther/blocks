use crate::renderer::depth_texture::DepthTexture;
use futures::executor::block_on;
use std::f32::consts::PI;
use wgpu::{Device, Queue, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct WgpuState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub depth_texture: DepthTexture,
}

impl WgpuState {
    // Creating some of the wgpu types requires async code
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is A handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU

        let (device, queue, surface) = WgpuState::get_device_queue_surface(window);
        let sc_desc = WgpuState::get_sc_desc(size);

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let depth_texture = DepthTexture::create_depth_texture(&device, &sc_desc, "depth_texture");
        //compute.compute_pass(&device, &queue);
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            depth_texture,
        }
    }
    pub fn get_device_queue_surface(window: &Window) -> (Device, Queue, Surface) {
        block_on(async {
            let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
            let surface = unsafe { instance.create_surface(window) };
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                })
                .await
                .unwrap();
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("requested device"),
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                    },
                    None,
                )
                .await
                .unwrap();
            return (device, queue, surface);
        })
    }
    pub fn get_sc_desc(size: PhysicalSize<u32>) -> wgpu::SwapChainDescriptor {
        wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.depth_texture = DepthTexture::create_depth_texture(
            &self.device,
            &self.sc_desc,
            "depth_texture",
        );
    }
}

pub fn gen_perspective_mat(size: (u32, u32)) -> [[f32; 4]; 4] {
    let (width, height) = size;
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = PI / 3.0;
    let zfar = 1024.0;
    let znear = 0.0;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 1.0],
    ]
}
