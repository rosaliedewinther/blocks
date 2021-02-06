use crate::positions::ChunkPos;
use crate::renderer::chunk_render_data::ChunkRenderData;
use crate::renderer::wgpu::WgpuState;
use crate::renderer::wgpu_pipeline::WgpuPipeline;
use futures::executor::block_on;
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
        let wgpu = block_on(WgpuState::new(&window));
        pipelines.insert(
            "main".to_string(),
            WgpuPipeline::new(&wgpu.device, &wgpu.sc_desc),
        );
        Renderer { pipelines, wgpu }
    }
    pub fn do_render_pass(
        &mut self,
        render_data: &HashMap<ChunkPos, ChunkRenderData>,
    ) -> Result<(), SwapChainError> {
        let frame = self.wgpu.swap_chain.get_current_frame()?.output;
        let mut encoder =
            self.wgpu
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
                    attachment: &self.wgpu.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            let pipeline = self.pipelines.get_mut("main").unwrap();
            pipeline.setup_render_pass(&mut render_pass);

            render_data.iter().for_each(|(_, data)| {
                data.do_render_pass(&mut render_pass);
            });
        }
        // submit will accept anything that implements IntoIter
        self.wgpu.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}
