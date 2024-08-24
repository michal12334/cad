use glium::glutin::surface::WindowSurface;
use glium::{BlendingFunction, Display, DrawParameters, Frame, LinearBlendingFactor, Program, Surface};

use math::vector4::Vector4;

use crate::drawing::domain::bezier_c0::BezierC0;

pub struct BezierC0Drawer {
    program: Program,
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
            uniform float t_min;
            uniform float t_max;
            
            void main() {
                if (mode == 0 && gl_PrimitiveIDIn == number_of_primitives - 1) {
                    for (float i = 0.0; i <= 1.00; i += 0.01) {
                        float t = t_min + i * (t_max - t_min);
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
                    for (float i = 0.0; i <= 1.00; i += 0.01) {
                        float t = t_min + i * (t_max - t_min);
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
                    for (float i = 0.0; i <= 1.00; i += 0.01) {
                        float t = t_min + i * (t_max - t_min);
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

        Self {
            program,
        }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        bezier: &BezierC0,
        perspective: &math::matrix4::Matrix4,
        view_matrix: &math::matrix4::Matrix4,
        color: [f32; 4],
        width: u32,
        height: u32,
        drawing_parameters: &DrawParameters,
    ) {
        if bezier.points.len() < 2 {
            return;
        }

        let max_distance = bezier.points.iter().fold(
            (0f32, 0f32, Vector4::new(0.0, 0.0, 0.0, 0.0)),
            |(max_x, max_y, prev), p| {
                let current = perspective.clone()
                    * view_matrix.clone()
                    * Vector4::new(p.position[0], p.position[1], p.position[2], 1.0);
                if prev.w == 0.0 {
                    return (max_x, max_y, current);
                }
                let distance = current.to_vector3() - prev.to_vector3();
                let distance_x = distance.x * width as f32 / 2.0;
                let distance_y = distance.y * height as f32 / 2.0;
                (max_x.max(distance_x), max_y.max(distance_y), current)
            },
        );

        let len = bezier.points.len();

        let number_of_draw_calls =
            (max_distance.0.max(max_distance.1) as u32).min(height.max(width)) / 50;

        let number_of_draw_calls = number_of_draw_calls.max(1);

        for i in 0..number_of_draw_calls {
            target
                .draw(
                    bezier.vertex_buffer.as_ref().unwrap(),
                    bezier.curve_index_buffer.as_ref().unwrap(),
                    &self.program,
                    &uniform! {
                        perspective: perspective.data,
                        view: view_matrix.data,
                        obj_color: color,
                        mode: len as i32 % 3,
                        number_of_primitives: (len as i32 - 1) / 3 + 1,
                        t_min: i as f32 / number_of_draw_calls as f32,
                        t_max: (i + 1) as f32 / number_of_draw_calls as f32,
                    },
                    drawing_parameters,
                )
                .unwrap();
        }
    }
}
