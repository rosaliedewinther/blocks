[[builtin(global_invocation_id)]]
var global_id: vec3<u32>;

[[block]]
struct PrimeIndices {
    data: [[stride(4)]] array<u32>;
}; // this is used as both input and output for convenience

[[group(0), binding(0)]]
var<storage> v_indices: [[access(read_write)]] PrimeIndices;

fn collatz_iterations(n_base: u32) -> u32{
    return n_base*n_base;
}

[[stage(compute), workgroup_size(1)]]
fn main() {
    v_indices.data[global_id.x] = collatz_iterations(v_indices.data[global_id.x]);
}