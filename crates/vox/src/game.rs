use crate::personal_world::PersonalWorld;
use std::time::Instant;
use vox_render::compute_renderer::renderer::Renderer;
use vox_render::compute_renderer::wgpu_state::WgpuState;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use winit_window_control::input::input::Input;
use winit_window_control::main_loop::{
    main_loop_run, Game, InitResult, RenderResult, UpdateResult,
};

pub struct VoxGame {
    personal_world: Option<PersonalWorld>,
    wgpu_state: Option<WgpuState>,
    renderer: Option<Renderer>,
    tick_duration: f64,
    render_duration: f64,
    frame_duration: f64,
}

impl VoxGame {
    pub fn new() -> VoxGame {
        VoxGame {
            personal_world: None,
            renderer: None,
            tick_duration: 0.0,
            render_duration: 0.0,
            wgpu_state: None,
            frame_duration: 0.0,
        }
    }
    pub fn run(self) {
        main_loop_run(self, None, None);
    }
}

impl Game for VoxGame {
    fn on_tick(&mut self, _dt: f64) -> UpdateResult {
        let timer = Instant::now();
        let pw = &mut self.personal_world.as_mut().unwrap();

        pw.on_game_tick(0.1);
        self.tick_duration = timer.elapsed().as_secs_f64();
        UpdateResult::Continue
    }
    fn on_render(&mut self, input: &mut Input, dt: f64, window: &Window) -> RenderResult {
        let timer = Instant::now();
        self.frame_duration = dt;
        let pw = self.personal_world.as_mut().unwrap();

        pw.player.handle_input(input, &dt);
        let renderer = self.renderer.as_mut().unwrap();

        let wgpu_state = self.wgpu_state.as_ref().unwrap();

        pw.world_render_data
            .update_all_buffers(wgpu_state, &pw.player, dt);
        renderer.do_render_pass(wgpu_state, window, vec![&mut pw.world_render_data]);

        input.update();
        self.render_duration = timer.elapsed().as_secs_f64();
        println!(
            "tick: {} us, render: {} us, frame: {}",
            (self.tick_duration * 1000000.0) as u32,
            (self.render_duration * 1000000.0) as u32,
            (self.frame_duration * 1000000.0) as u32
        );
        RenderResult::Continue
    }
    fn on_init(&mut self, window: &Window) -> InitResult {
        self.wgpu_state = Some(WgpuState::new(window));
        let renderer = Renderer::new(self.wgpu_state.as_mut().unwrap());
        self.personal_world = Some(PersonalWorld::new(
            window,
            &renderer,
            self.wgpu_state.as_ref().unwrap(),
        ));
        self.renderer = Some(renderer);
        self.wgpu_state.as_ref().unwrap().init_surface();

        InitResult::Continue
    }
    fn on_resize(&mut self, physical_size: PhysicalSize<u32>) {
        self.wgpu_state.as_mut().unwrap().resize(physical_size);
        let wgpu_state = self.wgpu_state.as_ref().unwrap();
        self.renderer.as_mut().unwrap().resized(wgpu_state);
    }
}
