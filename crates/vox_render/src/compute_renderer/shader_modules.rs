pub fn shader_module_init(filename: &str, device: &wgpu::Device) -> wgpu::ShaderModule {
    let file_text = std::fs::read(filename).unwrap();
    let fs_module_desc = wgpu::ShaderModuleDescriptor {
        label: Some(filename),
        source: wgpu::util::make_spirv(file_text.as_slice()),
    };
    device.create_shader_module(&fs_module_desc)
}
