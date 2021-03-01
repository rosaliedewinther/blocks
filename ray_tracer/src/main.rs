use crate::ray_tracer::RayTracer;

mod ray_tracer;
mod uniforms;
mod vertex;
mod wgpu;
mod wgpu_pipeline;

fn main() {
    let rt = RayTracer::new();
    rt.run();
}
