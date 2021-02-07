use crate::input::button::ButtonState;
use winit::event::VirtualKeyCode;

pub struct KeyboardState {
    going_down: Vec<VirtualKeyCode>,
    down: Vec<VirtualKeyCode>,
    released: Vec<VirtualKeyCode>,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            going_down: Vec::new(),
            down: Vec::new(),
            released: Vec::new(),
        }
    }
    pub fn update(&mut self) {
        self.down.extend(self.pressed.drain(..));
        self.released.clear();
    }
    pub fn pressed(&mut self, key: VirtualKeyCode) {
        self.going_down.push(key)
    }
    pub fn released(&mut self, key: VirtualKeyCode) {
        let mut index = self.down.iter().position(|k| k == key);
        if index.is_none() {
            index = self.going_down.iter().position(|k| k == key);
        }
        match index {
            None => {}
            Some(k) => {
                self.pressed.remove(k);
                self.released.push(key);
                return;
            }
        }
    }
    pub fn just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.going_down.iter().find(key).is_some()
    }
    pub fn down(&self, key: VirtualKeyCode) -> bool {
        self.going_down.iter().find(key).is_some() || self.down.iter().find(key).is_some()
    }
}
