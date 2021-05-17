use std::time::Instant;
use vox_world::positions::ChunkPos;
use vox_world::renderer::vertex::Vertex;
use vox_world::world::world::World;
use vox_world::world_gen::vertex_generation::get_chunk_vertices;
use wgpu::util::DeviceExt;
use wgpu::{Device, RenderPass};

pub struct ChunkRenderData {
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub num_vertices: Option<u32>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_indices: Option<u32>,
}

impl ChunkRenderData {
    pub fn new(world: &World, chunk_pos: &ChunkPos, device: &Device) -> ChunkRenderData {
        let timer = Instant::now();
        let (vertices, indices) = get_chunk_vertices(world, &chunk_pos);
        if vertices.len() == 0 {
            return ChunkRenderData {
                vertex_buffer: None,
                num_vertices: None,
                index_buffer: None,
                num_indices: None,
            };
        }
        println!("vertex gen time: {}", timer.elapsed().as_micros());
        let vertices: &[Vertex] = vertices.as_slice();
        let indices: &[u32] = indices.as_slice();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let num_indices = indices.len() as u32;
        let num_vertices = vertices.len() as u32;

        ChunkRenderData {
            vertex_buffer: Some(vertex_buffer),
            num_vertices: Some(num_vertices),
            index_buffer: Some(index_buffer),
            num_indices: Some(num_indices),
        }
    }
    pub fn do_render_pass<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        if self.num_indices.is_some() {
            render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.set_index_buffer(
                self.index_buffer.as_ref().unwrap().slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(0..self.num_indices.unwrap(), 0, 0..1);
        }
    }
}
