use crate::renderer::wgpu::gen_perspective_mat;
use nalgebra::Vector3;
use vox_core::constants::COLORS;
use vox_core::utils::get_rotation_matrix_y;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    viewer_pos: [f32; 3],
    time: f32,
    viewing_dir: [f32; 3],
    _padding1: f32,
    sun_dir: [f32; 3],
    _padding2: f32,
    colors: [[f32; 4]; 16],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            viewer_pos: [0.0, 0.0, 0.0],
            viewing_dir: [0.0, 1.0, 0.0],
            _padding1: 0.0,
            sun_dir: [0.0, 1.0, -1.0],
            _padding2: 0.0,
            time: 0.0,
            colors: COLORS,
        }
    }

    pub fn update_view_proj(
        &mut self,
        viewer_pos: [f32; 3],
        time_diff: f64,
        viewing_dir: [f32; 3],
    ) {
        self.time += time_diff as f32;
        self.viewer_pos = viewer_pos;
        self.viewing_dir = viewing_dir;
        let sun_dir = (get_rotation_matrix_y(self.time) * Vector3::new(1.0, -0.5, 0.0)).normalize();
        self.sun_dir = [sun_dir[0], sun_dir[1], sun_dir[2]];
    }
}
