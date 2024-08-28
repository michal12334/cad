use glium::glutin::surface::WindowSurface;
use glium::{Display, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer};

use backend::domain::vertex::Vertex;

pub struct PolygonDrawer {
    program: Program,
}

impl PolygonDrawer {
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

        let fragment_shader_src = r#"
            #version 460 core
    
            out vec4 color;
            
            uniform vec4 obj_color;
    
            void main() {
                color = obj_color;
            }
        "#;

        let program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Self { program }
    }

    pub fn draw<T: glium::index::Index>(
        &self,
        target: &mut Frame,
        vertex_buffer: &VertexBuffer<Vertex>,
        index_buffer: &IndexBuffer<T>,
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
        drawing_parameters: &DrawParameters,
    ) {
        target
            .draw(
                vertex_buffer,
                index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data,
                    view: view_matrix.data,
                    obj_color: color,
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
