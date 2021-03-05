use crate::ray_tracer::RayTracer;

mod ray_tracer;
mod uniforms;
mod vertex;
mod wgpu;
mod renderer;

fn main() {
    let rt = RayTracer::new();
    rt.run();
}
