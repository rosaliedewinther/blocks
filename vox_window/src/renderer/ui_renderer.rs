use winit::window::Window;
use egui_wgpu_backend::wgpu::Device;
use wgpu::{CommandEncoder, Queue, TextureView};
use crate::renderer::wgpu_state::WgpuState;
use egui::FontDefinitions;

pub struct UiRenderer{
    egui_ctx: egui::Context,
    egui_platform: egui_winit_platform::Platform,
    egui_render_pass: egui_wgpu_backend::RenderPass
}

impl UiRenderer{
    pub fn new(window: &Window, device: &Device) -> Self{
        Self{
            egui_ctx: egui::Context::default(),
            egui_platform: egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor{
                physical_width: window.inner_size().width,
                physical_height: window.inner_size().height,
                scale_factor: window.scale_factor(),
                font_definitions: egui::FontDefinitions::default(),
                style: Default::default()
            }),
            egui_render_pass: egui_wgpu_backend::RenderPass::new(device, crate::renderer::wgpu_state::WgpuState::get_render_texture_format(),1)
        }
    }
    pub fn draw_ui(&mut self, output_view: &TextureView, mut encoder: &mut CommandEncoder, wgpu_state: &WgpuState) {
        let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
            physical_width: wgpu_state.size.width,
            physical_height: wgpu_state.size.height,
            scale_factor: wgpu_state.scale_factor,
        };
        let (_output, paint_commands) = self.egui_platform.end_frame(None);
        let paint_jobs = self.egui_platform.context().tessellate(paint_commands);

        self.egui_render_pass.update_texture(&wgpu_state.device, &wgpu_state.queue, &self.egui_platform.context().texture());
        self.egui_render_pass.update_user_textures(&wgpu_state.device, &wgpu_state.queue);
        self.egui_render_pass.update_buffers(&wgpu_state.device, &wgpu_state.queue, paint_jobs.as_slice(), &screen_descriptor);

        // Record all render passes.
        self.egui_render_pass
            .execute(
                &mut encoder,
                &output_view,
                paint_jobs.as_slice(),
                &screen_descriptor,
                Some(wgpu::Color::BLACK),
            )
            .unwrap();
    }
}