use backend::domain::point::Point;
use backend::domain::vertex::Vertex;
use glium::glutin::surface::WindowSurface;
use glium::{Display, DrawParameters, Frame, Program, Surface};

pub struct BezierC0Drawer {
    program: Program,
    drawing_parameters: DrawParameters<'static>,
}

impl BezierC0Drawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 460 core
    
            in vec3 position;
            
            uniform mat4 perspective;
            uniform mat4 view;
    
            void main() {
                gl_Position = perspective * view * vec4(position, 1.0);
            }
        "#;

        let geometry_shader_src = r#"
            #version 460 core
            
            layout(lines_adjacency) in;
            layout(line_strip, max_vertices = 101) out;
            
            uniform int mode;
            uniform int number_of_primitives;
            
            void main() {
                if (mode == 0 && gl_PrimitiveIDIn == number_of_primitives - 1) {
                    for (float t = 0.0; t <= 1.00; t += 0.01) {
                        float it = 1.0 - t;
                        float b0 = it * it;
                        float b1 = 2.0 * it * t;
                        float b2 = t * t;
                        
                        vec4 position = 
                            gl_in[0].gl_Position * b0 
                            + gl_in[1].gl_Position * b1 
                            + gl_in[2].gl_Position * b2;
                        gl_Position = position;
                        EmitVertex();
                    }
                } else if (mode == 2 && gl_PrimitiveIDIn == number_of_primitives - 1) {
                    for (float t = 0.0; t <= 1.00; t += 0.01) {
                        float it = 1.0 - t;
                        float b0 = it;
                        float b1 = t;
                        
                        vec4 position = 
                            gl_in[0].gl_Position * b0 
                            + gl_in[1].gl_Position * b1;
                        gl_Position = position;
                        EmitVertex();
                    }
                } else {
                    for (float t = 0.0; t <= 1.00; t += 0.01) {
                        float it = 1.0 - t;
                        float b0 = it * it * it;
                        float b1 = 3.0 * it * it * t;
                        float b2 = 3.0 * it * t * t;
                        float b3 = t * t * t;
                        
                        vec4 position = 
                            gl_in[0].gl_Position * b0 
                            + gl_in[1].gl_Position * b1 
                            + gl_in[2].gl_Position * b2
                            + gl_in[3].gl_Position * b3;
                        gl_Position = position;
                        EmitVertex();
                    }
                }
                EndPrimitive();
            }
        "#;

        let fragment_shader_src = r#"
            #version 460 core
    
            out vec4 color;
            
            uniform vec4 obj_color;
    
            void main() {
                color = obj_color;
            }
        "#;

        let program = Program::from_source(
            display,
            vertex_shader_src,
            fragment_shader_src,
            Some(geometry_shader_src),
        )
        .unwrap();

        let mut drawing_parameters = DrawParameters::default();
        drawing_parameters.polygon_mode = glium::draw_parameters::PolygonMode::Line;
        drawing_parameters.line_width = Some(1.0);
        drawing_parameters.depth = glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        };

        Self {
            program,
            drawing_parameters,
        }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        display: &Display<WindowSurface>,
        points: &[Point],
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
    ) {
        let mut points = points
            .iter()
            .map(|p| Vertex {
                position: [
                    p.transformer.position.0 as f32,
                    p.transformer.position.1 as f32,
                    p.transformer.position.2 as f32,
                ],
            })
            .collect::<Vec<Vertex>>();
        let len = points.len();
        while points.len() % 3 != 1 {
            points.push(Vertex {
                position: [0.0, 0.0, 0.0],
            });
        }
        let vertex_buffer = glium::VertexBuffer::new(display, &points).unwrap();
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::LinesListAdjacency,
            &(0..(points.len() as u16 - 3))
                .step_by(3)
                .flat_map(|f| [f, f + 1, f + 2, f + 3])
                .collect::<Vec<u16>>(),
        )
        .unwrap();
        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.program,
                &uniform! {
                    perspective: perspective.data,
                    view: view_matrix.data,
                    obj_color: color,
                    mode: len as i32 % 3,
                    number_of_primitives: (len as i32 - 1) / 3 + 1,
                },
                &self.drawing_parameters,
            )
            .unwrap();
    }
}
