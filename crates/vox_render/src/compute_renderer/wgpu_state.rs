use futures::executor::block_on;
use wgpu::{Device, Queue, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct WgpuState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl WgpuState {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let (device, queue, surface) = WgpuState::get_device_queue_surface(window);
        let sc_desc = WgpuState::get_sc_desc(size);

        Self {
            surface,
            device,
            queue,
            sc_desc,
            size,
        }
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

            let limits = wgpu::Limits {
                max_storage_buffer_binding_size: 1073741824,
                ..Default::default()
            };
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("requested device"),
                        features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                        limits,
                    },
                    None,
                )
                .await
                .unwrap();

            (device, queue, surface)
        })
    }
    pub fn get_sc_desc(size: PhysicalSize<u32>) -> wgpu::SurfaceConfiguration {
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        }
    }
    pub fn init_surface(&self) {
        let surface_config = &self.sc_desc;
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
    }
}
