use crate::wgpu::WgpuState;
use crate::wgpu_pipeline::WgpuPipeline;
use std::time::Instant;
use winit::window::Window;
use winit_window_control::input::input::Input;
use winit_window_control::main_loop::{
    main_loop_run, Game, InitResult, RenderResult, UpdateResult,
};

pub struct RayTracer {
    wgpu: Option<WgpuState>,
    pipeline: Option<WgpuPipeline>,
}

impl RayTracer {
    pub fn new() -> RayTracer {
        RayTracer {
            wgpu: None,
            pipeline: None,
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
        self.pipeline = Some(WgpuPipeline::new(
            &self.wgpu.as_ref().unwrap().device,
            &self.wgpu.as_ref().unwrap().sc_desc,
        ));
        return InitResult::Continue;
    }
}
