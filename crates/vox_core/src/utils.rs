use nalgebra::base::Matrix3;
//use num_traits::real::Real;

pub fn get_rotation_matrix_z(angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        angle.cos(),
        -angle.sin(),
        0f64,
        angle.sin(),
        angle.cos(),
        0f64,
        0f64,
        0f64,
        1f64,
    )
}
pub fn get_rotation_matrix_x(angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        1f64,
        0f64,
        0f64,
        0f64,
        angle.cos(),
        -angle.sin(),
        0f64,
        angle.sin(),
        angle.cos(),
    )
}
pub fn get_rotation_matrix_y(angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        angle.cos(),
        0f64,
        angle.sin(),
        0f64,
        1f64,
        0f64,
        -angle.sin(),
        0f64,
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
