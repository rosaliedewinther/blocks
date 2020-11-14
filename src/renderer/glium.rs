use crate::constants::{HEIGHT, WIDTH};
use crate::player::Player;
use crate::renderer::vertex::Vertex;
use crate::utils::get_rotation_matrix_y;
use glium::backend::glutin::glutin::event_loop::EventLoop;
use glium::{
    glutin, Blend, Display, DrawError, DrawParameters, Frame, Program, Surface, VertexBuffer,
};
use nalgebra::Vector3;
use std::f32::consts::PI;
use std::time::SystemTime;

implement_vertex!(Vertex, position, color, normal);

pub struct DrawInfo<'a> {
    pub display: Display,
    pub program: Program,
    pub program_start: SystemTime,
    pub draw_params: DrawParameters<'a>,
}

pub fn draw_vertices(
    draw_info: &mut DrawInfo,
    target: &mut Frame,
    vertex_buffer: &VertexBuffer<Vertex>,
    player: &Player,
) -> Result<(), DrawError> {
    let time: f32 = draw_info.program_start.elapsed().unwrap().as_secs_f32();
    let rot_mat = get_rotation_matrix_y(time);
    let light_dir = rot_mat * Vector3::new(1.0, 0.3, 0.0);

    let uniforms = uniform! {
        view: player.get_view_matrix(),
        perspective: gen_persective_mat(target),
        viewer_pos: [light_dir[0],light_dir[1],light_dir[2]]
    };
    target.draw(
        vertex_buffer,
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &draw_info.program,
        &uniforms,
        &draw_info.draw_params,
    )
}

pub fn gen_persective_mat(target: &mut Frame) -> [[f32; 4]; 4] {
    let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = PI / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}

pub fn create_display(event_loop: &EventLoop<()>) -> Display {
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64));
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(23)
        .with_vsync(true)
        .with_multisampling(2);
    return glium::Display::new(wb, cb, &event_loop).unwrap();
}
pub fn gen_program(display: &Display) -> Program {
    let program = program!(display,
        140 => {

            vertex: "
                #version 150
                uniform mat4 perspective;
                uniform mat4 view;
                
                in vec3 position;
                in vec3 normal;
                in vec4 color;
                
                out vec3 vnormal;
                out vec4 vcolor;
                
                void main() {
                    vnormal = normal;
                    vcolor = color;
                    gl_Position = perspective * view * vec4(position, 1.0);
                }
            ",
            fragment: "
                #version 140
                uniform vec3 viewer_pos;
                
                in vec4 vcolor;
                in vec3 vnormal;
                
                out vec4 f_color;
                
                const vec3 diffuse_color = vec3(1.0, 1.0, 1.0);
                
                void main() {
                    float diffuse = max(dot(normalize(vnormal), normalize(viewer_pos)), 0.1);
                    vec4 new_color = vec4(vcolor[0]/255,vcolor[1]/255,vcolor[2]/255,vcolor[3]/255);
                    f_color = new_color * vec4(diffuse * diffuse_color, 1.0);
                }
            "
        },
    )
    .unwrap();
    return program;
}

pub fn gen_draw_params() -> DrawParameters<'static> {
    glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        blend: Blend::alpha_blending(),
        ..Default::default()
    }
}
