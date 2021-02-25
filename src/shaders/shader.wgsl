[[builtin(global_invocation_id)]]
var global_id: vec3<u32>;

[[block]]
struct ChunkData {
    data: [[stride(1)]] array<u8>;
};

[[block]]
struct Vertex {
    _pos: vec3<f32>,
    _color: u32,
    _normal: vec3<f32>,
    _type: u32,
}

[[block]]
struct ChunckVertexData{
    data: [[stride(32)]] array<Vertex>;
}

[[group(0), binding(0)]] var<storage> chunk_top: [[access(read_write)]] ChunkData;
[[group(0), binding(1)]] var<storage> chunk_bot: [[access(read_write)]] ChunkData;
[[group(0), binding(2)]] var<storage> chunk_front: [[access(read_write)]] ChunkData;
[[group(0), binding(3)]] var<storage> chunk_back: [[access(read_write)]] ChunkData;
[[group(0), binding(4)]] var<storage> chunk_left: [[access(read_write)]] ChunkData;
[[group(0), binding(5)]] var<storage> chunk_right: [[access(read_write)]] ChunkData;
[[group(0), binding(6)]] var<storage> chunk_main: [[access(read_write)]] ChunkData;
[[group(0), binding(7)]] var<storage> out: [[access(read_write)]] ChunkData;

[[stage(compute), workgroup_size(1)]]
fn main() {



}