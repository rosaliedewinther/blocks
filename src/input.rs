use device_query::{DeviceState, Keycode};
use enigo::{Enigo, MouseControllable};

pub struct Input{
    pub sensitivity_modifier: f32,
    pub prev_mouse_cords: (f32, f32),
    pub mouse_change: (f32, f32),
    pub enigo: Enigo,
    pub device_state: DeviceState,
    pub keys_pressed: Vec<Keycode>
}

impl Input {
    pub fn new() -> Input {
        let mut input = Input{
            sensitivity_modifier: 0.25,
            prev_mouse_cords: (0f32,0f32),
            mouse_change: (0f32, 0f32),
            enigo: Enigo::new(),
            device_state: DeviceState::new(),
            keys_pressed: Vec::new()
        };
        input.update();
        input.update();
        return input;
    }
    pub fn update(&mut self){
        let coords = self.device_state.query_pointer().coords;
        self.enigo.mouse_move_to(500, 200);
        self.mouse_change.0 = (coords.0 as f32 - self.prev_mouse_cords.0)*self.sensitivity_modifier;
        self.mouse_change.1 = (coords.1 as f32 - self.prev_mouse_cords.1)*self.sensitivity_modifier;
        self.keys_pressed = self.device_state.query_keymap();
        let old_coords = self.device_state.query_pointer().coords;
        self.prev_mouse_cords = (old_coords.0 as f32, old_coords.1 as f32);
    }
    pub fn key_pressed(&self, key: Keycode) -> bool{
        self.keys_pressed.contains(&key)
    }
    pub fn mouse_change(&self) -> (f32,f32){
        self.mouse_change
    }
}