use crate::ui::debug_info::DebugInfo;
use imgui::im_str;
use imgui::{Condition, FontSource};
use imgui_wgpu::Renderer as imgui_renderer;
use imgui_wgpu::RendererConfig;
use std::time::Instant;
use vox_render::renderer::renderer::Renderer;
use vox_render::renderer::wgpu::WgpuState;
use wgpu::{Device, Queue, RenderPass};
use winit::event::Event;
use winit::window::Window;
use winit_window_control::input::button::ButtonState;
use winit_window_control::input::input::Input;

pub struct UiRenderer {
    pub context: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_renderer,
    clear_color: wgpu::Color,
    last_frame: Instant,
    pub debug_info: DebugInfo,
}
impl UiRenderer {
    pub fn new(window: &Window, renderer: &Renderer) -> UiRenderer {
        let hidpi_factor = window.scale_factor();
        let mut context = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut context);
        platform.attach_window(
            context.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );

        let font_size = (13.0 * hidpi_factor) as f32;
        context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        context.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);
        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        let renderer_config = RendererConfig {
            texture_format: WgpuState::get_surface_desc(window.inner_size()).format,
            ..Default::default()
        };

        let renderer = imgui_renderer::new(
            &mut context,
            &renderer.wgpu.device,
            &renderer.wgpu.queue,
            renderer_config,
        );
        let last_frame = Instant::now();
        return UiRenderer {
            context,
            platform,
            renderer,
            clear_color,
            last_frame,
            debug_info: DebugInfo::new(100),
        };
    }
    pub fn update_input(&mut self, input: &Input) {
        self.context.io_mut().mouse_pos = input.mouse_state.mouse_location;
        self.context.io_mut().mouse_down[0] = input.mouse_state.get_left_button()
            == ButtonState::Down
            || input.mouse_state.get_left_button() == ButtonState::Pressed;
    }
    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        queue: &Queue,
        device: &Device,
        window: &Window,
    ) {
        let timediff = self.last_frame.elapsed();
        self.last_frame = Instant::now();
        self.context.io_mut().update_delta_time(timediff);
        let ui = self.context.frame();
        let debug_info = &self.debug_info;
        {
            let window = imgui::Window::new(im_str!("Hello too"));
            window
                .size([400.0, 200.0], Condition::FirstUseEver)
                .position([400.0, 200.0], Condition::FirstUseEver)
                .build(&ui, || debug_info.add_to_ui(&ui));
        }
        self.platform.prepare_render(&ui, &window);
        self.renderer
            .render(ui.render(), queue, device, render_pass)
            .unwrap();
    }
}
