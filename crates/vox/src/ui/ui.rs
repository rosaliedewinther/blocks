use crate::ui::debug_info::DebugInfo;
use imgui::im_str;
use imgui::{Condition, FontSource};
use imgui_wgpu::Renderer as imgui_renderer;
use imgui_wgpu::RendererConfig;
use std::time::Instant;
use vox_render::compute_renderer::renderer::Renderer;
use vox_render::compute_renderer::renderpassable::RenderPassable;
use vox_render::compute_renderer::wgpu_state::WgpuState;
use wgpu::{CommandEncoder, Device, Queue, RenderPass, SwapChainTexture};
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
    pub fn new(window: &Window, renderer: &Renderer, wgpu_state: &WgpuState) -> UiRenderer {
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
            texture_format: WgpuState::get_sc_desc(window.inner_size()).format,
            ..Default::default()
        };

        let renderer = imgui_renderer::new(
            &mut context,
            &wgpu_state.device,
            &wgpu_state.queue,
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
}

impl RenderPassable for UiRenderer {
    fn do_render_pass(
        &mut self,
        window: &Window,
        encoder: &mut CommandEncoder,
        wgpu_state: &WgpuState,
        frame: &SwapChainTexture,
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
        //self.platform.prepare_frame(ui.io_mut(), &window);
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass ui"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            self.renderer
                .render(
                    ui.render(),
                    &wgpu_state.queue,
                    &wgpu_state.device,
                    &mut render_pass,
                )
                .unwrap();
        }
    }
}
