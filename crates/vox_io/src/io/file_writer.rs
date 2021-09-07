use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_struct_to_file<T: bytemuck::Pod + bytemuck::Zeroable>(filename: &str, obj: &T) {
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(bytemuck::bytes_of(obj)).unwrap();
}
