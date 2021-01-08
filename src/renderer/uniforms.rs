use crate::player::Player;
use crate::renderer::wgpu::gen_perspective_mat;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    view: [[f32; 4]; 4],
    perspective: [[f32; 4]; 4],
    viewer_pos: [f32; 3],
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
        }
    }

    pub fn update_view_proj(&mut self, player: &Player, size: (u32, u32)) {
        let (width, height) = size;

        self.view = player.get_view_matrix();
        self.perspective = gen_perspective_mat((width, height));
        self.viewer_pos = [player.position.x, player.position.y, player.position.z]
    }
}
