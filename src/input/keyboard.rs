use std::collections::HashSet;
use winit::event::VirtualKeyCode;

pub struct KeyboardState {
    going_down: HashSet<VirtualKeyCode>,
    down: HashSet<VirtualKeyCode>,
    released: HashSet<VirtualKeyCode>,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            going_down: HashSet::new(),
            down: HashSet::new(),
            released: HashSet::new(),
        }
    }
    pub fn update(&mut self) {
        self.down.extend(self.going_down.drain());
        self.released.clear();
    }
    pub fn pressed(&mut self, key: VirtualKeyCode) {
        self.going_down.insert(key);
    }
    pub fn released(&mut self, key: VirtualKeyCode) {
        self.down.remove(&key);
        self.going_down.remove(&key);
    }
    pub fn just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.going_down.iter().find(|k| **k == key).is_some()
    }
    pub fn down(&self, key: VirtualKeyCode) -> bool {
        self.going_down.iter().find(|k| **k == key).is_some()
            || self.down.iter().find(|k| **k == key).is_some()
    }
}
