use glium::glutin::surface::WindowSurface;
use glium::{Display, DrawParameters, Frame, Program, Surface};

use backend::domain::torus::Torus;

pub struct TorusDrawer {
    program: Program,
}

impl TorusDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 140
    
            in vec3 position;
            
            uniform mat4 perspective;
            uniform mat4 model_matrix;
            uniform mat4 view;
    
            void main() {
                gl_Position = perspective * view * model_matrix * vec4(position, 1.0);
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
        torus: &Torus,
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
        drawing_parameters: &DrawParameters,
    ) {
        let vertex_buffer = glium::VertexBuffer::new(display, &torus.mesh.vertices).unwrap();
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::LinesList,
            &torus.mesh.indices,
        )
        .unwrap();
        let model_matrix = torus.transformer.get_model_matrix();
        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.program,
                &uniform! {
                    perspective: perspective.data,
                    model_matrix: model_matrix.data,
                    view: view_matrix.data,
                    obj_color: color
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
