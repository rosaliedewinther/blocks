use crate::compute_renderer::view_type::ViewType;
use nalgebra::Vector3;
use std::f64::consts::PI;
use vox_core::constants::{COLORS, WORLD_SIZE};
use vox_core::utils::get_rotation_matrix_y;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    viewer_pos: [f32; 3],
    time: f32,
    viewing_dir: [f32; 3],
    _padding1: f32,
    sun_dir: [f32; 3],
    world_size: u32,
    view_type: u32,
    _padding2: [f32; 3],
    ray_cast_data: [[f32; 4]; 3],
    colors: [[f32; 4]; 256],
}
impl Default for Uniforms {
    fn default() -> Self {
        Self::new()
    }
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            viewer_pos: [0.0, 0.0, 0.0],
            time: 0.0,
            viewing_dir: [0.0, 1.0, 0.0],
            _padding1: 0.0,
            sun_dir: [0.0, 1.0, -1.0],
            world_size: WORLD_SIZE as u32,
            view_type: 0,
            _padding2: [0.0, 0.0, 0.0],
            ray_cast_data: [[0.0; 4]; 3],
            colors: COLORS,
        }
    }

    pub fn update_view_proj(
        &mut self,
        viewer_pos: [f32; 3],
        time_diff: f64,
        viewing_dir: [f32; 3],
        screensize: [u32; 2],
        view_type: ViewType,
    ) {
        self.view_type = view_type as u32;
        self.time += time_diff as f32;
        self.viewer_pos = viewer_pos;
        let sun_dir =
            (get_rotation_matrix_y(self.time as f64) * Vector3::new(1.0, -0.5, 0.0)).normalize();
        self.sun_dir = [sun_dir[0] as f32, sun_dir[1] as f32, sun_dir[2] as f32];
        if (self.viewing_dir != viewing_dir) {
            let t = Vector3::new(
                viewing_dir[0] as f64,
                viewing_dir[1] as f64,
                viewing_dir[2] as f64,
            );
            let w = Vector3::new(0.0, -1.0, 0.0);
            let b = w.cross(&t);
            let tn = t.normalize();
            let bn = b.normalize();
            let vn = tn.cross(&bn);
            let gx = f64::tan(PI / 4.0);
            let gy = gx * (screensize[1] as f64 / screensize[0] as f64);
            let qx = ((2.0 * gx) / (screensize[0] as f64 - 1.0)) * bn;
            let qy = ((2.0 * gy) / (screensize[1] as f64 - 1.0)) * vn;
            let p1m = tn - gx * bn - gy * vn;
            self.ray_cast_data = [
                [qx[0] as f32, qx[1] as f32, qx[2] as f32, 0.0],
                [qy[0] as f32, qy[1] as f32, qy[2] as f32, 0.0],
                [p1m[0] as f32, p1m[1] as f32, p1m[2] as f32, 0.0],
            ];
        }
        self.viewing_dir = viewing_dir;
    }
}
