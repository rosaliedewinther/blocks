use crate::personal_world::PersonalWorld;
use std::time::Instant;
use vox_render::compute_renderer::renderer::Renderer;
use vox_render::compute_renderer::wgpu_state::WgpuState;
use vox_world::world::small_world::SmallWorld;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::window::Window;
use winit_window_control::input::input::Input;
use winit_window_control::main_loop::{
    main_loop_run, Game, InitResult, RenderResult, UpdateResult,
};

pub struct VoxGame {
    personal_world: Option<PersonalWorld>,
    wgpu_state: Option<WgpuState>,
    renderer: Option<Renderer>,
}

impl VoxGame {
    pub fn new() -> VoxGame {
        VoxGame {
            personal_world: None,
            renderer: None,
            wgpu_state: None,
        }
    }
    pub fn run(self) {
        main_loop_run(self, None, None);
    }
}

impl Game for VoxGame {
    fn on_tick(&mut self, dt: f64) -> UpdateResult {
        let pw = &mut self.personal_world.as_mut().unwrap();
        pw.ui
            .debug_info
            .set_numbers("player x".to_string(), pw.player.position.x as f64);
        pw.ui
            .debug_info
            .set_numbers("player y".to_string(), pw.player.position.y as f64);
        pw.ui
            .debug_info
            .set_numbers("player z".to_string(), pw.player.position.z as f64);
        pw.ui
            .debug_info
            .set_numbers("player view x".to_string(), pw.player.direction[0] as f64);
        pw.ui
            .debug_info
            .set_numbers("player view y".to_string(), pw.player.direction[1] as f64);
        pw.ui
            .debug_info
            .set_numbers("player view z".to_string(), pw.player.direction[2] as f64);

        let timer = Instant::now();

        pw.on_game_tick(0.1);
        pw.ui
            .debug_info
            .insert_stat("world tick".to_string(), timer.elapsed().as_secs_f32());

        return UpdateResult::Continue;
    }
    fn on_render(&mut self, input: &mut Input, dt: f64, window: &Window) -> RenderResult {
        let pw = self.personal_world.as_mut().unwrap();
        pw.ui
            .debug_info
            .insert_stat("fps".to_string(), (1.0 / dt) as f32);
        pw.update_ui_input(&input);
        pw.player.handle_input(&input, &(dt as f32));

        let renderer = self.renderer.as_mut().unwrap();

        let wgpu_state = self.wgpu_state.as_ref().unwrap();

        pw.world_render_data
            .update_all_buffers(&wgpu_state, &pw.player, dt);
        renderer.do_render_pass(
            &wgpu_state,
            window,
            vec![&mut pw.world_render_data, &mut pw.ui],
        );
        input.update();
        return RenderResult::Continue;
    }
    fn on_init(&mut self, window: &Window) -> InitResult {
        self.wgpu_state = Some(WgpuState::new(&window));
        let renderer = Renderer::new(&mut self.wgpu_state.as_mut().unwrap());
        self.personal_world = Some(PersonalWorld::new(
            window,
            &renderer,
            &self.wgpu_state.as_ref().unwrap(),
        ));
        self.renderer = Some(renderer);

        return InitResult::Continue;
    }
    fn on_resize(&mut self, physical_size: PhysicalSize<u32>) {}
}
