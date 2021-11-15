use crate::renderer::renderpassable::RenderPassable;
use crate::renderer::wgpu::WgpuState;
use crate::renderer::wgpu_pipeline::WgpuPipeline;
use std::collections::HashMap;
use wgpu::SwapChainError;
use winit::window::Window;

pub struct Renderer {
    pub pipelines: HashMap<String, WgpuPipeline>,
    pub wgpu: WgpuState,
}

impl Renderer {
    pub fn new(window: &Window) -> Renderer {
        let mut pipelines = HashMap::new();
        let wgpu = WgpuState::new(&window);
        pipelines.insert(
            "main".to_string(),
            WgpuPipeline::new(&wgpu.device, &wgpu.sc_desc),
        );
        Renderer { pipelines, wgpu }
    }

    pub fn do_render_pass<T: RenderPassable>(
        &self,
        window: &Window,
        obj: &mut T,
    ) -> Result<(), SwapChainError> {
        let frame = self.wgpu.swap_chain.get_current_frame()?.output;
        let mut encoder =
            self.wgpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        obj.do_render_pass(window, &mut encoder, &self.wgpu, &self.pipelines, &frame);
        self.wgpu.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

pub fn resize(new_size: winit::dpi::PhysicalSize<u32>, wgpu: &mut WgpuState) {
    wgpu.size = new_size;
    wgpu.sc_desc.width = new_size.width;
    wgpu.sc_desc.height = new_size.height;
    wgpu.swap_chain = wgpu.device.create_swap_chain(&wgpu.surface, &wgpu.sc_desc);
}
