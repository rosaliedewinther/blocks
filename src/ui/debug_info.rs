use imgui::im_str;
use imgui::{Condition, Ui, Window};
use std::collections::HashMap;

pub struct DebugInfo {
    stats: HashMap<String, (f32, f32, u32, Vec<f32>)>,
    numbers: HashMap<String, f64>,
    size: u32,
}

impl DebugInfo {
    pub fn new(size: u32) -> DebugInfo {
        DebugInfo {
            size,
            stats: HashMap::new(),
            numbers: HashMap::new(),
        }
    }
    pub fn add_to_ui(&self, ui: &Ui) {
        for (key, (max, min, _, arr)) in self.stats.iter() {
            let avg: f32 = arr.iter().sum::<f32>() / arr.len() as f32;
            ui.plot_lines(
                &*im_str!("{} max: {} min: {} avg: {}", key, max, min, avg),
                arr.as_slice(),
            )
            .scale_max(*max as f32)
            .scale_min(*min as f32)
            .graph_size([300.0, 150.0])
            .build();
        }
        for (key, val) in self.numbers.iter() {
            ui.text(&*im_str!("{}: {}", key, val));
        }
    }
    pub fn insert_stat(&mut self, name: String, value: f32) {
        let vec = self.stats.get_mut(&*name);
        match vec {
            None => {
                self.stats.insert(name, (value, value, 0, vec![value]));
            }
            Some((max, min, i, realvec)) => {
                if value < *min {
                    *min = value
                }
                if value > *max {
                    *max = value
                }
                if realvec.len() < self.size as usize {
                    realvec.push(value);
                } else {
                    realvec[*i as usize] = value;
                    *i += 1;
                    if *i == self.size {
                        *i = 0;
                    }
                }
            }
        };
    }
    pub fn set_numbers(&mut self, name: String, value: f64) {
        self.numbers.insert(name, value);
    }
}
