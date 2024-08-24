use glium::glutin::surface::WindowSurface;
use glium::{BlendingFunction, Display, DrawParameters, Frame, IndexBuffer, LinearBlendingFactor, Program, Surface, VertexBuffer};

use backend::domain::vertex::Vertex;

pub struct PointsDrawer {
    program: Program,
}

impl PointsDrawer {
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
            uniform int selected_index;
            uniform vec4 selected_color;
    
            void main() {
                if (gl_PrimitiveID == selected_index) {
                    color = selected_color;
                } else {
                    color = obj_color;
                }
            }
        "#;

        let program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Self {
            program,
        }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        vertex_buffer: &VertexBuffer<Vertex>,
        index_buffer: &IndexBuffer<u16>,
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
        selected_color: [f32; 4],
        selected_index: Option<usize>,
        drawing_parameters: &DrawParameters,
    ) {
        let mut drawing_parameters = drawing_parameters.clone();
        drawing_parameters.point_size = Some(6.0);
        
        target
            .draw(
                vertex_buffer,
                index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data,
                    view: view_matrix.data,
                    obj_color: color,
                    selected_index: if let Some(x) = selected_index { x as i32 } else { -1 },
                    selected_color: selected_color,
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
