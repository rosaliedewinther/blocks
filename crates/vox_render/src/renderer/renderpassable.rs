use crate::renderer::wgpu::WgpuState;
use crate::renderer::wgpu_pipeline::WgpuPipeline;
use std::collections::HashMap;
use wgpu::SwapChainTexture;
use winit::window::Window;

pub trait RenderPassable {
    fn do_render_pass(
        &mut self,
        window: &Window,
        encoder: &mut wgpu::CommandEncoder,
        wgpu_state: &WgpuState,
        pipelines: &HashMap<String, WgpuPipeline>,
        frame: &SwapChainTexture,
    );
}
