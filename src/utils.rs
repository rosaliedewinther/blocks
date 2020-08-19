use nalgebra::base::Matrix3;


pub fn get_rotation_matrix_z(angle: f32) -> Matrix3<f32>{
    Matrix3::new(
        angle.cos(),    -angle.sin(),   0f32,
        angle.sin(),    angle.cos(),    0f32,
        0f32,           0f32,           1f32
    )
}
pub fn get_rotation_matrix_x(angle: f32) -> Matrix3<f32>{
    Matrix3::new(
        1f32,   0f32,           0f32,
        0f32,   angle.cos(),    -angle.sin(),
        0f32,   angle.sin(),    angle.cos()
    )
}
pub fn get_rotation_matrix_y(angle: f32) -> Matrix3<f32>{
    Matrix3::new(
        angle.cos(),    0f32,   angle.sin(),
        0f32,           1f32,   0f32,
        -angle.sin(),   0f32,   angle.cos()
    )
}

pub fn negative_ceil(val: f32) -> f32{
    if val < 0f32 {
        return val.floor();
    } else {
        return val.ceil();
    }
}