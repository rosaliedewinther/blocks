use crate::compute_renderer::wgpu_state::WgpuState;
use winit::window::Window;

pub trait RenderPassable {
    fn do_render_pass(
        &mut self,
        window: &Window,
        encoder: &mut wgpu::CommandEncoder,
        wgpu_state: &WgpuState,
        frame: &wgpu::TextureView,
    );
}
