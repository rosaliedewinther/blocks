use crate::world::small_world::SmallWorld;
use nalgebra::{Matrix3, Vector3};
use std::f32::consts::PI;
use vox_core::constants::COLORS;
use vox_core::positions::{ChunkPos, ObjectPos};
use vox_core::utils::{get_rotation_matrix_y, get_rotation_matrix_z};
use winit::event::VirtualKeyCode;
use winit_window_control::input::input::Input;
#[derive(Debug)]
pub struct Player {
    pub position: ObjectPos,
    pub direction: Vector3<f64>,
    pub up: [f64; 3],
    pub speed: f64,
    pub camera_speed: f64,
    pub render_distance: f64,
    pub generated_chunks_for: ChunkPos,
    pub gravity: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: ObjectPos {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3::new(0f64, 0.0f64, 1.0f64),
            up: [0f64, 1.0f64, 0f64],
            speed: 10f64,
            camera_speed: 2.0f64,
            render_distance: 5000f64,
            generated_chunks_for: ChunkPos {
                x: i32::MAX,
                y: i32::MAX,
                z: i32::MAX,
            },
            gravity: 0.0,
        }
    }

    pub fn handle_input(&mut self, input: &Input, dt: &f64) {
        self.change_position(
            input,
            VirtualKeyCode::A,
            0.5f64 * PI as f64,
            *dt * self.speed as f64,
        );
        self.change_position(
            input,
            VirtualKeyCode::D,
            1.5f64 * PI as f64,
            *dt * self.speed as f64,
        );
        self.change_position(
            input,
            VirtualKeyCode::W,
            0f64 * PI as f64,
            *dt * self.speed as f64,
        );
        self.change_position(
            input,
            VirtualKeyCode::S,
            1f64 * PI as f64,
            *dt * self.speed as f64,
        );
        if input.key_pressed(VirtualKeyCode::Space) {
            let diff = *dt * self.speed as f64;
            self.position.y += diff;
            /*if !Player::collides(&self.position.get_diff(0.0, diff, 0.0), world) {
                self.position.y += diff;
            }*/
        }
        if input.key_pressed(VirtualKeyCode::LShift) {
            let diff = -*dt * self.speed as f64;
            self.position.y += diff;
            /*if !Player::collides(&self.position.get_diff(0.0, diff, 0.0), world) {

            }*/
        }

        let mouse_change = input.mouse_change();
        let xdiff = -mouse_change[0] as f64 * dt * self.camera_speed as f64;
        let ydiff = -mouse_change[1] as f64 * dt * self.camera_speed as f64;

        self.change_direction_vertical(ydiff);
        self.change_direction_horizontal(&get_rotation_matrix_y(xdiff));

        self.direction = self.direction.normalize();
    }
    pub fn change_position(
        &mut self,
        input: &Input,
        key: VirtualKeyCode,
        rotation_degree: f64,
        change: f64,
    ) {
        if input.key_pressed(key) {
            let move_vec = get_rotation_matrix_y(rotation_degree) * &self.direction;
            let to_extend =
                1f64 / (move_vec[0].powf(2f64).abs() + move_vec[2].powf(2f64).abs()).sqrt();
            let x_change = change * move_vec.x * to_extend;
            let z_change = change * move_vec.z * to_extend;
            self.position.x += x_change;
            self.position.z += z_change;
            /*if !Player::collides(&self.position.get_diff(x_change, 0.0, z_change), world) {

            }*/
        }
    }
    pub fn change_direction_horizontal(&mut self, mat: &Matrix3<f64>) {
        self.direction = mat * &self.direction;
    }
    pub fn change_direction_vertical(&mut self, change: f64) {
        let backup_dir = self.direction.clone();
        let angle = (backup_dir[2] / backup_dir[0]).atan() as f64;
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
        let blockpos = pos.get_block();
        let faceblock = world.get_block(blockpos);
        let feetblock = world.get_block(blockpos.get_diff(0, -1, 0));
        return if (faceblock.is_some() && COLORS[faceblock.unwrap() as usize][3] == 255.0)
            || (feetblock.is_some() && COLORS[feetblock.unwrap() as usize][3] == 255.0)
        {
            true
        } else {
            false
        };
    }
    pub fn chunk_in_view_distance(&self, pos: &ChunkPos) -> bool {
        self.position.get_chunk().get_distance(pos) < self.render_distance
    }
}
