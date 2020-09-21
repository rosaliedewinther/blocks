use crate::renderer::glium::DrawInfo;
use imgui_winit_support::{WinitPlatform, HiDpiMode};
use imgui_glium_renderer::imgui;

pub struct UiRenderer {
    pub context: imgui::Context,
    pub renderer: imgui_glium_renderer::Renderer,
    pub platform: WinitPlatform,
}



impl UiRenderer{
    pub fn init(draw_info: &DrawInfo) -> UiRenderer{
        let mut context = imgui::Context::create();
        let mut renderer = imgui_glium_renderer::Renderer::init(&mut context, &draw_info.display).unwrap();

        let mut platform = WinitPlatform::init(&mut context);
        {
            let gl_window = &draw_info.display.gl_window();
            let window = gl_window.window();
            platform.attach_window(context.io_mut(), &window, HiDpiMode::Rounded);
        }
        UiRenderer{context, renderer, platform}
    }
}