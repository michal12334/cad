use glium::glutin::surface::WindowSurface;
use glium::{Blend, Display, IndexBuffer, Program, Surface, VertexBuffer};

use backend::domain::vertex::Vertex;

pub struct InfiniteGridDrawer {
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
}

impl InfiniteGridDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 460 core
            
            in vec3 position;
            
            uniform mat4 perspective;
            uniform mat4 view;
            
            out vec3 nearPoint;
            out vec3 farPoint;
            out mat4 fragView;
            out mat4 fragProj;
            
            vec3 UnprojectPoint(float x, float y, float z, mat4 view, mat4 projection) {
                mat4 viewInv = inverse(view);
                mat4 projInv = inverse(projection);
                vec4 unprojectedPoint =  viewInv * projInv * vec4(x, y, z, 1.0);
                return unprojectedPoint.xyz / unprojectedPoint.w;
            }

            void main() {
                nearPoint = UnprojectPoint(position.x, position.y, 0.0, view, perspective).xyz;
                farPoint = UnprojectPoint(position.x, position.y, 1.0, view, perspective).xyz;
                fragView = view;
                fragProj = perspective;
                gl_Position = vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 460 core
            
            float near = 0.01;
            float far = 100;
            in vec3 nearPoint;
            in vec3 farPoint;
            in mat4 fragView;
            in mat4 fragProj;
            out vec4 outColor;
            
            vec4 grid(vec3 fragPos3D, float scale) {
                vec2 coord = fragPos3D.xz * scale;
                vec2 derivative = fwidth(coord);
                vec2 grid = abs(fract(coord - 0.5) - 0.5) / derivative;
                float line = min(grid.x, grid.y);
                float minimumz = min(derivative.y, 1);
                float minimumx = min(derivative.x, 1);
                vec4 color = vec4(0.2, 0.2, 0.2, 1.0 - min(line, 1.0));
                // z axis
                if(fragPos3D.x > -0.1 * minimumx && fragPos3D.x < 0.1 * minimumx)
                    color.z = 1.0;
                // x axis
                if(fragPos3D.z > -0.1 * minimumz && fragPos3D.z < 0.1 * minimumz)
                    color.x = 1.0;
                return color;
            }
            float computeDepth(vec3 pos) {
                vec4 clip_space_pos = fragProj * fragView * vec4(pos.xyz, 1.0);
                return (clip_space_pos.z / clip_space_pos.w);
            }
            float computeLinearDepth(vec3 pos) {
                vec4 clip_space_pos = fragProj * fragView * vec4(pos.xyz, 1.0);
                float clip_space_depth = (clip_space_pos.z / clip_space_pos.w) * 2.0 - 1.0; // put back between -1 and 1
                float linearDepth = (2.0 * near * far) / (far + near - clip_space_depth * (far - near)); // get linear value between 0.01 and 100
                return linearDepth / far; // normalize
            }
            void main() {
                float t = -nearPoint.y / (farPoint.y - nearPoint.y);
                vec3 fragPos3D = nearPoint + t * (farPoint - nearPoint);
            
                gl_FragDepth = ((gl_DepthRange.diff * computeDepth(fragPos3D)) +
                    gl_DepthRange.near + gl_DepthRange.far) / 2.0;
            
                float linearDepth = computeLinearDepth(fragPos3D);
                float fading = max(0, (0.5 - linearDepth));
            
                outColor = (grid(fragPos3D, 10) + grid(fragPos3D, 1))* float(t > 0); // adding multiple resolution for the grid
                outColor.a *= fading;
            }
        "#;

        let program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Self {
            program,
            vertex_buffer: VertexBuffer::new(
                display,
                &[
                    Vertex {
                        position: [1.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [1.0, -1.0, 0.0],
                    },
                    Vertex {
                        position: [-1.0, -1.0, 0.0],
                    },
                    Vertex {
                        position: [-1.0, 1.0, 0.0],
                    },
                ],
            )
            .unwrap(),
            index_buffer: IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &[0u16, 2, 3, 2, 0, 1],
            )
            .unwrap(),
        }
    }

    pub fn draw(
        &self,
        target: &mut glium::Frame,
        perspective: &[[f32; 4]; 4],
        view: &[[f32; 4]; 4],
    ) {
        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniform! {
                    perspective: *perspective,
                    view: *view,
                },
                &draw_params,
            )
            .unwrap();
    }
}
