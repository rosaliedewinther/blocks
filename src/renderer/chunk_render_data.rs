use crate::positions::{ChunkPos, LocalChunkPos};
use crate::renderer::vertex::Vertex;
use crate::world::World;
use crate::world_gen::chunk::Chunk;
use crate::world_gen::meta_chunk::MetaChunk;
use crate::world_gen::vertex_generation::get_chunk_vertices;
use wgpu::util::DeviceExt;
use wgpu::{Device, RenderPass};

pub struct ChunkRenderData {
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl ChunkRenderData {
    pub fn new(world: &World, chunk_pos: &ChunkPos, device: &Device) -> ChunkRenderData {
        let (mut vertices, indices) = get_chunk_vertices(world, &chunk_pos);
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
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
        }
    }
    pub fn do_render_pass<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..));
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
