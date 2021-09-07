use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_struct_from_file<T: bytemuck::Pod + bytemuck::Zeroable>(
    filename: &str,
) -> Option<Box<T>> {
    println!("ooooo");
    let f = File::open(filename);
    return match f {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf).unwrap();
            Some(Box::new(*bytemuck::from_bytes_mut(&mut buf[..])))
        }
        Err(_) => None,
    };
}
