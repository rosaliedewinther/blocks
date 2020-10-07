use crate::renderer::glium::DrawInfo;
use glium::Frame;
use imgui::im_str;
use imgui_glium_renderer::imgui;
use imgui_glium_renderer::imgui::{Condition, Window};
use imgui_winit_support::{HiDpiMode, WinitPlatform};

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
    pub fn draw(&mut self, draw_info: &DrawInfo, strings: &Vec<String>, target: &mut Frame) {
        let mut ui = self.context.frame();

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
        self.renderer.render(target, ui.render());
    }
}
