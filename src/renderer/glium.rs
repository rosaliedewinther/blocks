use crate::constants::{HEIGHT, WIDTH};
use crate::player::Player;
use crate::renderer::vertex::{Normal, Vertex};
use glium::backend::glutin::glutin::event_loop::EventLoop;
use glium::{glutin, Blend, Display, DrawParameters, Frame, Program, Surface, VertexBuffer};
use std::f32::consts::PI;
use std::time::SystemTime;

implement_vertex!(Vertex, position, color);
implement_vertex!(Normal, normal);

pub struct DrawInfo<'a> {
    pub display: Display,
    pub program: Program,
    pub program_start: SystemTime,
    pub draw_params: DrawParameters<'a>,
}

pub fn draw_vertices(
    draw_info: &mut DrawInfo,
    target: &mut Frame,
    buffers: &(VertexBuffer<Vertex>, VertexBuffer<Normal>),
    player: &Player,
) {
    let utime: f32 = draw_info.program_start.elapsed().unwrap().as_secs_f32();
    let perspective = {
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
    };
    let view = player.get_view_matrix();
    let light = [1.0, 0.4, 0.9f32];
    let uniforms = uniform! {
        matrix: [
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [0.0, 0.0, 1.0, 1.0f32]
        ],
        view: view,
        time: utime,
        perspective: perspective,
        light: light,
        camera_direction: [player.direction.x, player.direction.y, player.direction.z],
    };
    // drawing a frame
    let vertex_buffer = &buffers.0;
    let normal_buffer = &buffers.1;
    match target.draw(
        (vertex_buffer, normal_buffer),
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &draw_info.program,
        &uniforms,
        &draw_info.draw_params,
    ) {
        Ok(_) => (),
        Err(err) => println!("{}", err.to_string()),
    }
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
                uniform mat4 matrix;
                uniform mat4 view;
                uniform float time;
                uniform vec3 u_light;
                in vec3 position;
                in vec3 normal;
                out vec3 v_normal;
                
                in vec4 color;
                out vec4 vColor;
                //out vec3 v_normal;
                void main() {
                    v_normal = normal;
                    gl_Position = vec4(position, 1.0);
                    gl_Position = perspective * view * gl_Position;
                    vColor = color;
                }
            ",
            fragment: "
                #version 140
                in vec4 vColor;
                //in vec3 v_normal;
                in vec3 v_normal;
                out vec4 f_color;
                uniform vec3 u_light;
                uniform vec3 camera_direction;
                in vec3 v_position;
                
                
                
                const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
                const vec3 diffuse_color = vec3(1.0, 1.0, 1.0);
                const vec3 specular_color = vec3(1.0, 1.0, 1.0);
                
                void main() {
                    //vec3 u_light = vec3(1.4, 0.4, 0.7);
                    vec3 camera_direction2 = vec3(camera_direction[0],-camera_direction[1], camera_direction[2]);
                    float diffuse = max(dot(normalize(v_normal), normalize(camera_direction2)), 0.0);
                    
                    
                    vec3 camera_dir = normalize(-v_position);
                    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
                    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
                    f_color = vColor + vec4(diffuse * diffuse_color + specular * specular_color, 1.0);
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
