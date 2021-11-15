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
    pub surface_desc: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub depth_texture: DepthTexture,
}

impl WgpuState {
    // Creating some of the wgpu types requires async code
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let (device, queue, surface) = WgpuState::get_device_queue_surface(window);
        let surface_desc = WgpuState::get_surface_desc(size);
        let depth_texture = DepthTexture::create_depth_texture(&device, &surface_desc, "depth_texture");
        let s = Self {
            surface,
            device,
            queue,
            surface_desc,
            size,
            depth_texture,
        };
        s.init_surface();
        s
    }
    pub fn get_device_queue_surface(window: &Window) -> (Device, Queue, Surface) {
        block_on(async {
            let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
            let surface = unsafe { instance.create_surface(window) };
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
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
    pub fn init_surface(&self){
        let surface_config = &self.surface_desc;
        self.surface.configure(&self.device, surface_config);
    }
    pub fn get_surface_desc(size: PhysicalSize<u32>) -> wgpu::SurfaceConfiguration {
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.surface_desc.width = new_size.width;
        self.surface_desc.height = new_size.height;
        self.depth_texture = DepthTexture::create_depth_texture(
            &self.device,
            &self.surface_desc,
            "depth_texture",
        );
        self.init_surface();
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
