use crate::world::small_world::SmallWorld;
use nalgebra::{Matrix3, Vector3};
use std::f32::consts::PI;
use vox_core::constants::COLORS;
use vox_core::positions::{ChunkPos, ObjectPos};
use vox_core::utils::{get_rotation_matrix_y, get_rotation_matrix_z};
use winit::event::VirtualKeyCode;
use winit_window_control::input::input::Input;

pub struct Player {
    pub position: ObjectPos,
    pub direction: Vector3<f32>,
    pub up: [f32; 3],
    pub speed: f32,
    pub camera_speed: f32,
    pub render_distance: f32,
    pub generated_chunks_for: ChunkPos,
    pub gravity: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: ObjectPos {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3::new(0f32, 0.0f32, 1.0f32),
            up: [0f32, 1.0f32, 0f32],
            speed: 100f32,
            camera_speed: 0.5f32,
            render_distance: 5000f32,
            generated_chunks_for: ChunkPos {
                x: i32::max_value(),
                y: i32::max_value(),
                z: i32::max_value(),
            },
            gravity: 0.0,
        }
    }

    pub fn handle_input(&mut self, input: &Input, dt: &f32, world: &SmallWorld) {
        self.change_position(
            input,
            VirtualKeyCode::A,
            1.5f32 * PI,
            *dt * self.speed,
            world,
        );
        self.change_position(
            input,
            VirtualKeyCode::D,
            0.5f32 * PI,
            *dt * self.speed,
            world,
        );
        self.change_position(input, VirtualKeyCode::W, 0f32 * PI, *dt * self.speed, world);
        self.change_position(input, VirtualKeyCode::S, 1f32 * PI, *dt * self.speed, world);
        if input.key_pressed(VirtualKeyCode::Space) {
            let diff = *dt * self.speed;
            if !Player::collides(&self.position.get_diff(0.0, diff, 0.0), world) {
                self.position.y += diff;
            }
        }
        if input.key_pressed(VirtualKeyCode::LShift) {
            let diff = -*dt * self.speed;
            if !Player::collides(&self.position.get_diff(0.0, diff, 0.0), world) {
                self.position.y += diff;
            }
        }

        let mouse_change = input.mouse_change();
        let xdiff = mouse_change[0] * dt * self.camera_speed;
        let ydiff = -mouse_change[1] * dt * self.camera_speed;

        if ydiff < 0f32 {
            self.change_direction_vertical(ydiff);
        } else {
            self.change_direction_vertical(ydiff);
        }
        if xdiff < 0f32 {
            self.change_direction_horizontal(&get_rotation_matrix_y(xdiff));
        } else {
            self.change_direction_horizontal(&get_rotation_matrix_y(xdiff));
        }
    }

    pub fn change_position(
        &mut self,
        input: &Input,
        key: VirtualKeyCode,
        rotation_degree: f32,
        change: f32,
        world: &SmallWorld,
    ) {
        if input.key_pressed(key) {
            let move_vec = get_rotation_matrix_y(rotation_degree) * &self.direction;
            let to_extend =
                1f32 / (move_vec[0].powf(2f32).abs() + move_vec[2].powf(2f32).abs()).sqrt();
            let x_change = change * move_vec.x * to_extend;
            let z_change = change * move_vec.z * to_extend;
            if !Player::collides(&self.position.get_diff(x_change, 0.0, z_change), world) {
                self.position.x += x_change;
                self.position.z += z_change;
            }
        }
    }
    pub fn change_direction_horizontal(&mut self, mat: &Matrix3<f32>) {
        self.direction = mat * &self.direction;
    }
    pub fn change_direction_vertical(&mut self, change: f32) {
        let mut backup_dir = self.direction.clone();
        backup_dir[1] = 0f32;
        let angle = (backup_dir[2] / backup_dir[0]).atan();
        if backup_dir[0].is_sign_negative() {
            self.direction = get_rotation_matrix_y(-angle)
                * get_rotation_matrix_z(-change)
                * get_rotation_matrix_y(angle)
                * &self.direction;
        } else {
            self.direction = get_rotation_matrix_y(-angle)
                * get_rotation_matrix_z(change)
                * get_rotation_matrix_y(angle)
                * &self.direction;
        }
        if backup_dir[0].is_sign_positive() != self.direction[0].is_sign_positive() {
            self.direction[0] = backup_dir[0];
        }
        if backup_dir[2].is_sign_positive() != self.direction[2].is_sign_positive() {
            self.direction[2] = backup_dir[2];
        }
    }

    pub fn update(&mut self, _dt: &f32, world: &SmallWorld) {
        loop {
            if Player::collides(&self.position, world) {
                self.position.y += 1.0;
            } else {
                return;
            }
        }
    }
    //pub fn get_collision_points() -> [ObjectPos; 8] {}
    pub fn collides(pos: &ObjectPos, world: &SmallWorld) -> bool {
        /*let blockpos = pos.get_block();
        let faceblock = world.get_block(blockpos);
        let feetblock = world.get_block(blockpos.get_diff(0, -1, 0));
        return if (faceblock.is_some() && COLORS[faceblock.unwrap() as usize][3] == 255.0)
            || (feetblock.is_some() && COLORS[feetblock.unwrap() as usize][3] == 255.0)
        {
            true
        } else {
            false
        };*/
        return false
    }

    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };

        let s = [
            self.up[1] * f[2] - self.up[2] * f[1],
            self.up[2] * f[0] - self.up[0] * f[2],
            self.up[0] * f[1] - self.up[1] * f[0],
        ];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

        let u = [
            f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0],
        ];

        let p = [
            -self.position.x * s_norm[0]
                - self.position.y * s_norm[1]
                - self.position.z * s_norm[2],
            -self.position.x * u[0] - self.position.y * u[1] - self.position.z * u[2],
            -self.position.x * f[0] - self.position.y * f[1] - self.position.z * f[2],
        ];

        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
    pub fn chunk_in_view_distance(&self, pos: &ChunkPos) -> bool {
        self.position.get_chunk().get_distance(pos) < self.render_distance
    }
}
