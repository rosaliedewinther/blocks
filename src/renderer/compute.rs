use std::borrow::Cow;
use wgpu::Device;

struct Compute {}

impl Compute {
    pub fn new(device: &Device) -> Compute {
        let cs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shaders/shader.wgsl"))),
            flags: wgpu::ShaderFlags::VALIDATION,
        });

        return Compute {};
    }
}
