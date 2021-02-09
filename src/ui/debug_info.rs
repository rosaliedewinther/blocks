use imgui::im_str;
use imgui::{Condition, Ui, Window};
use std::collections::HashMap;

pub struct DebugInfo {
    stats: HashMap<String, (f32, f32, Vec<f32>)>,
    size: u32,
}

impl DebugInfo {
    pub fn new(size: u32) -> DebugInfo {
        DebugInfo {
            size,
            stats: HashMap::new(),
        }
    }
    pub fn add_to_ui(&self, ui: &Ui) {
        for (key, (max, min, arr)) in self.stats.iter() {
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
    }
    pub fn insert_stat(&mut self, name: String, value: f32) {
        let vec = self.stats.get_mut(&*name);
        match vec {
            None => {
                self.stats.insert(name, (value, value, vec![value]));
            }
            Some((max, min, realvec)) => {
                if value < *min {
                    *min = value
                }
                if value > *max {
                    *max = value
                }
                realvec.push(value);
                if realvec.len() > self.size as usize {
                    realvec.remove(0);
                }
            }
        };
    }
}
