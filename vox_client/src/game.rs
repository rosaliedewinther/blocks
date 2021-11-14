use vox_window::main_loop::{UpdateResult, RenderResult, InitResult, Game};
use vox_window::input::input::Input;

pub struct VoxGame {}

impl Game for VoxGame {
    fn on_tick(&mut self, input: &Input, _dt: f64) -> UpdateResult {
        UpdateResult::Continue
    }
    fn on_frame(&mut self, input: &Input, _dt: f64) -> RenderResult {
        RenderResult::Continue
    }
    fn on_init(&mut self) -> InitResult {
        InitResult::Continue
    }
}
