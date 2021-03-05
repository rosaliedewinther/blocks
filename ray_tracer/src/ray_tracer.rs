use crate::renderer::Renderer;
use crate::wgpu::WgpuState;
use image::GenericImageView;
use std::time::Instant;
use wgpu::BindGroup;
use winit::window::Window;
use winit_window_control::input::input::Input;
use winit_window_control::main_loop::{
    main_loop_run, Game, InitResult, RenderResult, UpdateResult,
};

pub struct RayTracer {
    wgpu: Option<WgpuState>,
    renderer: Option<Renderer>,
}

impl RayTracer {
    pub fn new() -> RayTracer {
        RayTracer {
            wgpu: None,
            renderer: None,
        }
    }
    pub fn run(self) {
        main_loop_run(self);
    }
}

impl Game for RayTracer {
    fn on_tick(&mut self, dt: f64) -> UpdateResult {
        return UpdateResult::Continue;
    }
    fn on_render(&mut self, input: &mut Input, dt: f64, window: &Window) -> RenderResult {
        self.renderer
            .as_ref()
            .unwrap()
            .do_render_pass(self.wgpu.as_ref().unwrap());
        RenderResult::Continue
    }
    fn on_init(&mut self, window: &Window) -> InitResult {
        self.wgpu = Some(WgpuState::new(window));
        self.renderer = Some(Renderer::new(self.wgpu.as_mut().unwrap()));

        return InitResult::Continue;
    }
}
