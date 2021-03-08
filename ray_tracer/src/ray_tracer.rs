use crate::renderer::Renderer;
use crate::wgpu::WgpuState;
use std::time::Instant;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use winit_window_control::input::input::Input;
use winit_window_control::main_loop::{
    main_loop_run, Game, InitResult, RenderResult, UpdateResult,
};

pub struct RayTracer {
    wgpu: Option<WgpuState>,
    renderer: Option<Renderer>,
    timer: Instant,
}

impl RayTracer {
    pub fn new() -> RayTracer {
        RayTracer {
            wgpu: None,
            renderer: None,
            timer: Instant::now(),
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
    fn on_resize(&mut self, physical_size: PhysicalSize<u32>) {
        self.renderer
            .as_mut()
            .unwrap()
            .resized(self.wgpu.as_mut().unwrap());
    }
    fn on_render(&mut self, input: &mut Input, dt: f64, window: &Window) -> RenderResult {
        self.renderer.as_mut().unwrap().update(dt);
        self.renderer
            .as_ref()
            .unwrap()
            .do_render_pass(self.wgpu.as_ref().unwrap());
        println!("fps estimate: {}", 1.0 / self.timer.elapsed().as_secs_f32());
        self.timer = Instant::now();
        RenderResult::Continue
    }
    fn on_init(&mut self, window: &Window) -> InitResult {
        self.wgpu = Some(WgpuState::new(window));
        self.renderer = Some(Renderer::new(self.wgpu.as_mut().unwrap()));

        return InitResult::Continue;
    }
}
