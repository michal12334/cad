use backend::domain::vertex::Vertex;
use glium::glutin::surface::WindowSurface;
use glium::{Display, DrawParameters, Frame, Program, Surface};

use backend::domain::intersection::Intersection;

pub struct IntersectionDrawer {
    program: Program,
}

impl IntersectionDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 140
    
            in vec3 position;
            
            uniform mat4 perspective;
            uniform mat4 view;
    
            void main() {
                gl_Position = perspective * view * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
    
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

    pub fn draw(
        &self,
        target: &mut Frame,
        display: &Display<WindowSurface>,
        intersection: &Intersection,
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
        drawing_parameters: &DrawParameters,
    ) {
        let vertex_buffer = glium::VertexBuffer::new(
            display,
            &intersection
                .intersection_points
                .iter()
                .map(|p| Vertex {
                    position: [p.x, p.y, p.z],
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::LinesList,
            &(0..intersection.intersection_points.len() as u32).collect::<Vec<_>>(),
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
                    obj_color: color
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
