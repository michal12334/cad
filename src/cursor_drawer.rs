use glium::{Display, DrawParameters, Frame, Program, Surface};
use glium::glutin::surface::WindowSurface;
use backend::domain::cursor::Cursor;
use backend::domain::point::Point;
use backend::domain::vertex::Vertex;

pub struct CursorDrawer {
    program: Program,
    drawing_parameters: DrawParameters<'static>,
}

impl CursorDrawer {
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
    
            void main() {
                color = vec4(0.0, 1.0, 0.0, 1.0);
            }
        "#;

        let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
        
        let mut drawing_parameters = DrawParameters::default();
        drawing_parameters.polygon_mode = glium::draw_parameters::PolygonMode::Line;
        drawing_parameters.line_width = Some(4.0);
        drawing_parameters.depth = glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        };
        
        Self { program, drawing_parameters }
    }
    
    pub fn draw(&self, target: &mut Frame, display: &Display<WindowSurface>, cursor: &Cursor, perspective: &math::matrix4::Matrix4, view_matrix: &math::matrix4::Matrix4) {
        let vertex_buffer = glium::VertexBuffer::new(display, &cursor.mesh.vertices).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::LinesList, &cursor.mesh.indices).unwrap();
        let model_matrix = cursor.transformer.get_model_matrix();
        target.draw(
            &vertex_buffer,
            &indices,
            &self.program,
            &uniform! {
                perspective: perspective.data,
                model_matrix: model_matrix.data,
                view: view_matrix.data,
            },
            &self.drawing_parameters)
            .unwrap();
    }
}
