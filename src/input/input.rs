use crate::input::keyboard::KeyboardState;
use crate::input::mouse::MouseState;
use device_query::{DeviceState, Keycode};
use enigo::Enigo;
use winit::dpi::PhysicalPosition;
use winit::event::{
    ElementState, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode,
};
use winit::event_loop::ControlFlow;

pub struct Input {
    pub sensitivity_modifier: f32,
    pub mouse_state: MouseState,
    pub cursor_in_screen: bool,
    pub keyboard_state: KeyboardState,
}

impl Input {
    pub fn new() -> Input {
        let mut input = Input {
            sensitivity_modifier: 0.25,
            mouse_state: MouseState::new(),
            keyboard_state: KeyboardState::new(),
            cursor_in_screen: true,
        };
        input.update();
        input.update();
        return input;
    }
    pub fn update_cursor_moved(&mut self, pos: &PhysicalPosition<f64>) {
        self.mouse_state.mouse_delta = [
            pos.x as f32 - self.mouse_state.mouse_location[0],
            pos.y as f32 - self.mouse_state.mouse_location[1],
        ];
        self.mouse_state.mouse_location = [pos.x as f32, pos.y as f32];
    }
    pub fn update_cursor_entered(&mut self) {
        self.cursor_in_screen = true;
    }
    pub fn update_cursor_left(&mut self) {
        self.cursor_in_screen = false;
    }
    pub fn update_mouse_input(&mut self, state: &ElementState, button: &MouseButton) {
        match state {
            ElementState::Pressed => match button {
                MouseButton::Left => self.mouse_state.left_button_pressed(),
                MouseButton::Right => self.mouse_state.right_button_pressed(),
                _ => {}
            },
            ElementState::Released => match button {
                MouseButton::Left => self.mouse_state.left_button_released(),
                MouseButton::Right => self.mouse_state.right_button_released(),
                _ => {}
            },
        }
    }
    pub fn update_mouse_wheel(&mut self, delta: &MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, scrolled) => {
                self.mouse_state.scroll_delta = *scrolled as f64;
                self.mouse_state.scroll_location += *scrolled as f64;
            }
            MouseScrollDelta::PixelDelta(z) => {}
        }
    }
    pub fn update_keyboard_input(&mut self, input: &KeyboardInput, control_flow: &mut ControlFlow) {
        match input {
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    }

    pub fn update(&mut self) {
        //let coords = self.device_state.query_pointer().coords;
        //self.enigo.mouse_move_to(400, 400);
        /*self.mouse_change.0 =
            (coords.0 as f32 - self.prev_mouse_cords.0) * self.sensitivity_modifier;
        self.mouse_change.1 =
            (coords.1 as f32 - self.prev_mouse_cords.1) * self.sensitivity_modifier;
        self.keys_pressed = self.device_state.query_keymap();
        let old_coords = self.device_state.query_pointer().coords;
        self.prev_mouse_cords = (old_coords.0 as f32, old_coords.1 as f32);*/
    }
    pub fn key_pressed(&self, key: Keycode) -> bool {
        self.keys_pressed.contains(&key)
    }
    pub fn mouse_change(&self) -> (f32, f32) {
        self.mouse_delta
    }
}
