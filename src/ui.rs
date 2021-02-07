use crate::renderer::renderer::Renderer;
use crate::renderer::wgpu::WgpuState;
use device_query::DeviceState;
use enigo::Enigo;
use imgui::{im_str, Condition, FontSource};
use imgui_wgpu::Renderer as imgui_renderer;
use imgui_wgpu::RendererConfig;
use std::time::Instant;
use wgpu::{Device, Queue, RenderPass};
use winit::event::Event;
use winit::window::Window;

pub struct UiRenderer {
    context: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_renderer,
    clear_color: wgpu::Color,
    last_frame: Instant,
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
        context.set_ini_filename(None);

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

        let mut renderer = imgui_renderer::new(
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
        };
    }
    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        queue: &Queue,
        device: &Device,
        window: &Window,
        event: &Event<()>,
    ) {
        self.platform
            .handle_event(self.context.io_mut(), &window, event);
        let timediff = self.last_frame.elapsed();
        self.last_frame = Instant::now();
        self.context.io_mut().update_delta_time(timediff);
        self.platform
            .prepare_frame(self.context.io_mut(), &window)
            .expect("Failed to prepare frame");
        let input = DeviceState::new();
        self.context.io_mut().mouse_pos = [
            input.query_pointer().coords.0 as f32,
            input.query_pointer().coords.1 as f32,
        ];

        self.context.io_mut().mouse_down = [
            input.query_pointer().button_pressed[1],
            input.query_pointer().button_pressed[1],
            input.query_pointer().button_pressed[1],
            input.query_pointer().button_pressed[1],
            input.query_pointer().button_pressed[1],
        ];
        let ui = self.context.frame();

        {
            let window = imgui::Window::new(im_str!("Hello world"));
            window
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("Hello world!"));
                    ui.text(im_str!("This...is...imgui-rs on WGPU!"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(im_str!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0],
                        mouse_pos[1]
                    ));
                });

            let window = imgui::Window::new(im_str!("Hello too"));
            window
                .size([400.0, 200.0], Condition::FirstUseEver)
                .position([400.0, 200.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("Frametime: {:?}", timediff));
                });

            ui.show_demo_window(&mut true);
        }
        self.platform.prepare_render(&ui, &window);
        self.renderer
            .render(ui.render(), queue, device, render_pass);
    }
}

/*
pub struct UiRenderer {
    pub context: imgui::Context,
    pub renderer: imgui_glium_renderer::Renderer,
    pub platform: WinitPlatform,
}

impl UiRenderer {
    pub fn init(draw_info: &DrawInfo) -> UiRenderer {
        let mut context = imgui::Context::create();
        let renderer =
            imgui_glium_renderer::Renderer::init(&mut context, &draw_info.display).unwrap();

        let mut platform = WinitPlatform::init(&mut context);
        {
            let gl_window = &draw_info.display.gl_window();
            let window = gl_window.window();
            platform.attach_window(context.io_mut(), &window, HiDpiMode::Rounded);
        }
        UiRenderer {
            context,
            renderer,
            platform,
        }
    }
    pub fn draw(
        &mut self,
        draw_info: &DrawInfo,
        strings: &Vec<String>,
        target: &mut Frame,
    ) -> Result<(), RendererError> {
        let ui = self.context.frame();

        Window::new(im_str!("it just works"))
            .size([300.0, 200.0], Condition::FirstUseEver)
            .opened(&mut false)
            .build(&ui, || {
                for line in strings.iter() {
                    ui.text(line);
                }
            });

        self.platform
            .prepare_render(&ui, draw_info.display.gl_window().window());
        self.renderer.render(target, ui.render())
    }
}
*/
