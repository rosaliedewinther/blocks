use crate::renderer::wgpu::gen_perspective_mat;
use nalgebra::Vector3;
use vox_core::constants::COLORS;
use vox_core::utils::get_rotation_matrix_y;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    view: [[f32; 4]; 4],
    perspective: [[f32; 4]; 4],
    viewer_pos: [f32; 3],
    time: f32,
    sun_dir: [f32; 3],
    _padding: f32,
    colors: [[f32; 4]; 16],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
            perspective: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
            viewer_pos: [0.0, 0.0, 0.0],
            sun_dir: [0.0, 0.0, 0.0],
            time: 0.0,
            _padding: 0.0,
            colors: COLORS,
        }
    }

    pub fn update_view_proj(
        &mut self,
        viewer_pos: [f32; 3],
        view_matrix: [[f32; 4]; 4],
        size: (u32, u32),
        time: f64,
    ) {
        let (width, height) = size;
        self.view = view_matrix;
        self.perspective = gen_perspective_mat((width, height));
        self.viewer_pos = viewer_pos;
        let sun_dir = get_rotation_matrix_y(time as f32) * Vector3::new(1.0, 1.0, 0.0);
        self.sun_dir = [sun_dir[0], sun_dir[1], sun_dir[2]];
        self.time = time as f32;
    }
}
