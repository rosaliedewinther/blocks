use crate::renderer::renderpassable::RenderPassable;
use crate::renderer::wgpu::WgpuState;
use crate::renderer::wgpu_pipeline::WgpuPipeline;
use std::collections::HashMap;
use winit::window::Window;
use wgpu::TextureViewDescriptor;

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
            WgpuPipeline::new(&wgpu.device, &wgpu.surface_desc),
        );
        Renderer { pipelines, wgpu }
    }

    pub fn do_render_pass<T: RenderPassable>(
        &self,
        window: &Window,
        obj: &mut T,
    ) -> Result<(), wgpu::SurfaceError> {
        let frame = self.wgpu.surface.get_current_texture()?;
        {
            let texture_view = frame.texture.create_view(&Default::default());
            let mut encoder =
                self.wgpu
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });
            obj.do_render_pass(window, &mut encoder, &self.wgpu, &self.pipelines, &texture_view);
            self.wgpu.queue.submit(std::iter::once(encoder.finish()));
        }
        wgpu::SurfaceTexture::present(frame);
        Ok(())
    }
}
