use futures::executor::block_on;
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
}

impl WgpuState {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let (device, queue, surface) = WgpuState::get_device_queue_surface(window);
        let sc_desc = WgpuState::get_sc_desc(size);

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        //compute.compute_pass(&device, &queue);
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
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

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}
