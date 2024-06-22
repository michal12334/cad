use glium::glutin::surface::WindowSurface;
use glium::{Display, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer};

use backend::domain::vertex::Vertex;

pub struct PointsDrawer {
    program: Program,
    drawing_parameters: DrawParameters<'static>,
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
    
            void main() {
                color = obj_color;
            }
        "#;

        let program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let mut drawing_parameters = DrawParameters::default();
        drawing_parameters.polygon_mode = glium::draw_parameters::PolygonMode::Point;
        drawing_parameters.point_size = Some(6.0);
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
        vertex_buffer: &VertexBuffer<Vertex>,
        index_buffer: &IndexBuffer<u16>,
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
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
                &self.drawing_parameters,
            )
            .unwrap();
    }
}
