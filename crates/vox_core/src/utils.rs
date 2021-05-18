use nalgebra::base::Matrix3;
//use num_traits::real::Real;

pub fn get_rotation_matrix_z(angle: f32) -> Matrix3<f32> {
    Matrix3::new(
        angle.cos(),
        -angle.sin(),
        0f32,
        angle.sin(),
        angle.cos(),
        0f32,
        0f32,
        0f32,
        1f32,
    )
}
pub fn get_rotation_matrix_x(angle: f32) -> Matrix3<f32> {
    Matrix3::new(
        1f32,
        0f32,
        0f32,
        0f32,
        angle.cos(),
        -angle.sin(),
        0f32,
        angle.sin(),
        angle.cos(),
    )
}
pub fn get_rotation_matrix_y(angle: f32) -> Matrix3<f32> {
    Matrix3::new(
        angle.cos(),
        0f32,
        angle.sin(),
        0f32,
        1f32,
        0f32,
        -angle.sin(),
        0f32,
        angle.cos(),
    )
}

pub fn negative_floor(val: f32) -> i32 {
    if val > 0f32 {
        val.floor() as i32
    } else {
        val.ceil() as i32
    }
}
pub fn wrap(val: i32, max: i32) -> i32 {
    if val < 0 {
        ((val % max) + max) % max
    } else {
        val % max
    }
}
pub fn wrapf(val: f32, max: f32) -> f32 {
    if val < 0f32 {
        ((val % max) + max) % max
    } else {
        val % max
    }
}
pub fn to_sign_of(read: i32, write: i32) -> i32 {
    return if read.is_negative() {
        -(write.abs())
    } else {
        write.abs()
    };
}
