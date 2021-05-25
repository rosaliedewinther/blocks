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

        let timer = Instant::now();

        pw.on_game_tick(0.1);
        pw.ui
            .debug_info
            .insert_stat("world tick".to_string(), timer.elapsed().as_secs_f32());
        return UpdateResult::Continue;
    }
    fn on_render(&mut self, input: &mut Input, dt: f64, window: &Window) -> RenderResult {
        self.renderer.as_mut().unwrap().update(dt);
        self.renderer
            .as_mut()
            .unwrap()
            .do_render_pass(&self.wgpu_state.as_ref().unwrap());

        return RenderResult::Continue;
        let timer = Instant::now();
        let pw = self.personal_world.as_mut().unwrap();
        let timer = Instant::now();
        let number_generated = pw.check_vertices_to_generate(self.renderer.as_ref().unwrap());
        if number_generated > 0 {
            pw.ui.debug_info.insert_stat(
                "per chunk vertex time".to_string(),
                timer.elapsed().as_secs_f32() / number_generated as f32,
            );
        }

        pw.update_ui_input(&input);
        pw.player.handle_input(&input, &(dt as f32), &pw.world);
        if pw.render(&window, self.renderer.as_mut().unwrap()) == RenderResult::Exit {
            return RenderResult::Exit;
        }
        input.update();
        self.personal_world
            .as_mut()
            .unwrap()
            .ui
            .debug_info
            .insert_stat("render time".to_string(), timer.elapsed().as_secs_f32());
        RenderResult::Continue
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
