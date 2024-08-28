use glium::glutin::surface::WindowSurface;
use glium::program::SourceCode;
use glium::{Display, DrawParameters, Frame, Program, Surface};

use crate::drawing::domain::surface_c2::SurfaceC2;

pub struct SurfaceC2Drawer {
    program: Program,
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

            uniform bool swap_xy;
            uniform bool is_cylinder;

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

                float x2 = x * x;
                float x3 = x2 * x;
                float b0x = (-x3 + 3 * x2 - 3 * x + 1) / 6;
                float b1x = (3 * x3 - 6 * x2 + 4) / 6;
                float b2x = (-3 * x3 + 3 * x2 + 3 * x + 1) / 6;
                float b3x = (x3) / 6;

                float y2 = y * y;
                float y3 = y2 * y;
                float b0y = (-y3 + 3 * y2 - 3 * y + 1) / 6;
                float b1y = (3 * y3 - 6 * y2 + 4) / 6;
                float b2y = (-3 * y3 + 3 * y2 + 3 * y + 1) / 6;
                float b3y = (y3) / 6;

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

                float x = gl_TessCoord.x;
                float y = gl_TessCoord.y * float(gl_TessLevelOuter[0]) / float(gl_TessLevelOuter[0] - 1);

                if (swap_xy && !is_cylinder) {
                    float temp = x;
                    x = y;
                    y = temp;
                } else if (swap_xy && is_cylinder) {
                    y = gl_TessCoord.x;
                    x = gl_TessCoord.y;
                }

                gl_Position = get_bernstein_value(x, y, positions);
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
        surface: &SurfaceC2,
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
        tess_level: u8,
        drawing_parameters: &DrawParameters,
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
                    swap_xy: false,
                    is_cylinder: surface.is_cylinder,
                },
                &drawing_parameters,
            )
            .unwrap();
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
                    swap_xy: true,
                    is_cylinder: surface.is_cylinder,
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
