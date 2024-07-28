use glium::glutin::surface::WindowSurface;
use glium::{Display, DrawParameters, Frame, Program, Surface};
use glium::program::SourceCode;

use math::vector4::Vector4;

use crate::drawing::domain::surface_c2::SurfaceC2;

pub struct SurfaceC2Drawer {
    program: Program,
    drawing_parameters: DrawParameters<'static>,
}

impl SurfaceC2Drawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader = r#"
            #version 460 core
    
            in vec3 position;
            
            uniform mat4 perspective;
            uniform mat4 view;
    
            void main() {
                gl_Position = perspective * view * vec4(position, 1.0);
            }
        "#;

        let fragment_shader = r#"
            #version 460 core
    
            out vec4 color;
            
            uniform vec4 obj_color;
    
            void main() {
                color = obj_color;
            }
        "#;
        
        let tessellation_control_shader = r#"
            #version 460 core

            layout(vertices = 16) out;

            uniform int tess_level;

            void main() {
                gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

                gl_TessLevelOuter[0] = tess_level;
                gl_TessLevelOuter[1] = tess_level;
            }
        "#;

        let tessellation_evaluation_shader = r#"
            #version 460 core

            layout(isolines, equal_spacing) in;

            vec4 get_bernstein_value(float x, float y, vec4 positions[16]) {
                vec4 v11 = positions[0];
                vec4 v21 = positions[1];
                vec4 v31 = positions[2];
                vec4 v41 = positions[3];
                vec4 v12 = positions[4];
                vec4 v22 = positions[5];
                vec4 v32 = positions[6];
                vec4 v42 = positions[7];
                vec4 v13 = positions[8];
                vec4 v23 = positions[9];
                vec4 v33 = positions[10];
                vec4 v43 = positions[11];
                vec4 v14 = positions[12];
                vec4 v24 = positions[13];
                vec4 v34 = positions[14];
                vec4 v44 = positions[15];

                float ix = 1.0 - x;
                float b0x = ix * ix * ix;
                float b1x = 3.0 * ix * ix * x;
                float b2x = 3.0 * ix * x * x;
                float b3x = x * x * x;

                float iy = 1.0 - y;
                float b0y = iy * iy * iy;
                float b1y = 3.0 * iy * iy * y;
                float b2y = 3.0 * iy * y * y;
                float b3y = y * y * y;

                vec4 p = b0x * (b0y * v11 + b1y * v12 + b2y * v13 + b3y * v14) +
                         b1x * (b0y * v21 + b1y * v22 + b2y * v23 + b3y * v24) +
                         b2x * (b0y * v31 + b1y * v32 + b2y * v33 + b3y * v34) +
                         b3x * (b0y * v41 + b1y * v42 + b2y * v43 + b3y * v44);
            
                return p;
            }

            void main() {
                vec4 positions[16];
                for (int i = 0; i < 16; i++) {
                    positions[i] = gl_in[i].gl_Position;
                }
                float y = gl_TessCoord.y * float(gl_TessLevelOuter[0]) / float(gl_TessLevelOuter[0] - 1);
                gl_Position = get_bernstein_value(gl_TessCoord.x, y, positions);
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
        surface: &SurfaceC2,
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
        tess_level: u8,
    ) {
        target
            .draw(
                &surface.vertex_buffer,
                &surface.surface_index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data,
                    view: view_matrix.data,
                    obj_color: color,
                    tess_level: tess_level as i32,
                },
                &self.drawing_parameters,
            )
            .unwrap();
    }
}
