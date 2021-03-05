use crate::wgpu::WgpuState;
use image::GenericImageView;
use std::time::Instant;
use winit::window::Window;
use winit_window_control::input::input::Input;
use winit_window_control::main_loop::{
    main_loop_run, Game, InitResult, RenderResult, UpdateResult,
};
use wgpu::BindGroup;
use crate::renderer::Renderer;

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





        RenderResult::Continue
    }
    fn on_init(&mut self, window: &Window) -> InitResult {
        self.wgpu = Some(WgpuState::new(window));
        self.renderer = Some(Renderer::new(&mut self.wgpu.as_ref().unwrap()));


        return InitResult::Continue;
    }
}
