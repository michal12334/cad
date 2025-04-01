use backend::domain::gregory::Gregory;
use backend::domain::vertex::Vertex;
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::program::SourceCode;
use glium::{Display, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer};

pub struct GregoryDrawer {
    program: Program,
}

impl GregoryDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader = r#"
            #version 410 core

            in vec3 position;
              
            void main() {
                gl_Position = vec4(position, 1.0);
            }
        "#;

        let fragment_shader = r#"
            #version 410 core
    
            out vec4 color;
            
            uniform vec4 obj_color;
    
            void main() {
                color = obj_color;
            }
        "#;

        let tessellation_control_shader = r#"
            #version 410 core

            layout(vertices = 20) out;

            uniform int tess_level;

            void main() {
                gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

                gl_TessLevelOuter[0] = tess_level;
                gl_TessLevelOuter[1] = tess_level;
            }
        "#;

        let tessellation_evaluation_shader = r#"
            #version 410 core

            layout(isolines, equal_spacing) in;

            uniform mat4 perspective;
            uniform mat4 view;
            uniform bool swap_xy;
            
            vec3 p(uint idx) {
                return gl_in[idx].gl_Position.xyz;
            }

            vec3 bezier3(vec3 b0, vec3 b1, vec3 b2, vec3 b3, float t) {
                float t1 = 1.0f - t;

                b0 = t1 * b0 + t * b1;
                b1 = t1 * b1 + t * b2;
                b2 = t1 * b2 + t * b3;

                b0 = t1 * b0 + t * b1;
                b1 = t1 * b1 + t * b2;

                return t1 * b0 + t * b1;
            }

            const float eps = 1e-10;

            vec3 gregory(float u, float v) {
                vec3 pi00 = (u * p(12) + v * p(16)) / (u + v + eps);
                vec3 pi01 = (u * p(13) + (1.0 - v) * p(17)) / (u + 1.0 - v + eps);
                vec3 pi10 = ((1.0 - u) * p(14) + v * p(18)) / (1.0 - u + v + eps);
                vec3 pi11 = ((1.0 - u) * p(15) + (1.0 - v) * p(19)) / (2.0 - u - v + eps);

                vec3 p0 = bezier3(p(0), p(1), p(2), p(3), v);
                vec3 p1 = bezier3(p(4), pi00, pi01, p(5), v);
                vec3 p2 = bezier3(p(6), pi10, pi11, p(7), v);
                vec3 p3 = bezier3(p(8), p(9), p(10), p(11), v);

                return bezier3(p0, p1, p2, p3, u);
            }


            void main() {
                float u = gl_TessCoord.x;
                float v = gl_TessCoord.y;

                if (swap_xy) {
                    float temp = u;
                    u = v;
                    v = temp;
                }

                vec4 position = vec4(gregory(u, v), 1.0f);

                gl_Position = perspective * view * position;
            }
        "#;

        let program = Program::new(
            display,
            SourceCode {
                vertex_shader,
                fragment_shader,
                tessellation_control_shader: Some(tessellation_control_shader),
                tessellation_evaluation_shader: Some(tessellation_evaluation_shader),
                geometry_shader: None,
            },
        )
        .unwrap();

        Self { program }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        gregory: &Gregory,
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
        tess_level: u8,
        drawing_parameters: &DrawParameters,
        display: &Display<WindowSurface>,
    ) {
        let vertex_buffer = VertexBuffer::new(
            display,
            &gregory
                .patches
                .iter()
                .flat_map(|p| {
                    p.top
                        .iter()
                        .chain(p.top_sides.iter())
                        .chain(p.bottom_sides.iter())
                        .chain(p.bottom.iter())
                        .chain(p.u_inner.iter())
                        .chain(p.v_inner.iter())
                })
                .map(|p| Vertex {
                    position: [p.x, p.y, p.z],
                })
                .collect::<Vec<Vertex>>(),
        )
        .unwrap();

        let index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::Patches {
                vertices_per_patch: 20,
            },
            &(0..(20 * gregory.patches.len()))
                .map(|x| x as u16)
                .collect::<Vec<_>>(),
        )
        .unwrap();

        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data,
                    view: view_matrix.data,
                    obj_color: color,
                    tess_level: tess_level as i32,
                    swap_xy: false,
                },
                &drawing_parameters,
            )
            .unwrap();

        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data,
                    view: view_matrix.data,
                    obj_color: color,
                    tess_level: tess_level as i32,
                    swap_xy: true,
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
