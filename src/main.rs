mod vertex;
mod mesh;
mod torus;
mod transformer;

#[macro_use]
extern crate glium;

use std::ops::DerefMut;
use glium::{DrawParameters, Surface};
use glium::vertex::MultiVerticesSource;
use winit::{event, event_loop};
use user_interface::ui::Ui;
use crate::vertex::Vertex;
extern crate user_interface;

fn main() {
    let mut width = 800;
    let mut height = 600;
    
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("CAD")
        .with_inner_size(width, height)
        .build(&event_loop);

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &window, &event_loop);
    
    let mut torus = torus::Torus {
        major_radius: 0.5,
        minor_radius: 0.25,
        major_segments: 32,
        minor_segments: 16,
    };
    let shape = mesh::Mesh::from_torus(&torus);
    
    let transformer = transformer::Transformer {
        position: (0.5, 0.0, 2.0),
        rotation: (1.0, -1.0, 0.0),
        scale: (1.0, 1.0, 1.0),
    };
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape.vertices).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &shape.indices).unwrap();
    
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        
        uniform mat4 perspective;
        uniform mat4 model_matrix;

        void main() {
            gl_Position = perspective * model_matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    let mut ui = Ui::new();
    
    event_loop.run(move |event, _window_target, control_flow| {
        let mut redraw = || {
            let repaint_after = egui_glium.run(&window, ui.build());

            *control_flow = if repaint_after.is_zero() {
                window.request_redraw();
                event_loop::ControlFlow::Poll
            } else if let Some(repaint_after_instant) =
                std::time::Instant::now().checked_add(repaint_after)
            {
                event_loop::ControlFlow::WaitUntil(repaint_after_instant)
            } else {
                event_loop::ControlFlow::Wait
            };

            {
                let model_matrix = transformer.get_model_matrix();
                
                let perspective = {
                    let aspect_ratio = height as f32 / (width - 200) as f32;

                    let fov: f32 = std::f32::consts::PI / 3.0;
                    let zfar = 1024.0;
                    let znear = 0.1;

                    let f = 1.0 / (fov / 2.0).tan();

                    [
                        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                        [         0.0         ,     f ,              0.0              ,   0.0],
                        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                    ]
                };
                
                let mut target = display.draw();
                
                let mut drawing_parameters = DrawParameters::default();
                drawing_parameters.viewport = Some(glium::Rect {
                    left: 200,
                    bottom: 0,
                    width: width - 200,
                    height: height,
                });
                drawing_parameters.polygon_mode = glium::draw_parameters::PolygonMode::Line;

                target.clear_color(0.0, 0.0, 1.0, 1.0);

                egui_glium.paint(&display, &mut target);

                target.draw(&vertex_buffer, &indices, &program, &uniform! { perspective: perspective, model_matrix: model_matrix }, &drawing_parameters).unwrap();

                target.finish().unwrap();
            }
        };
        match event {
            event::Event::RedrawRequested(_) => redraw(),

            event::Event::WindowEvent { event, .. } => {
                use event::WindowEvent;
                match &event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    WindowEvent::Resized(new_size) => {
                        display.resize((*new_size).into());
                        width = new_size.width;
                        height = new_size.height;
                    }
                    _ => {}
                }

                let event_response = egui_glium.on_event(&event);

                if event_response.repaint {
                    window.request_redraw();
                }
            }
            event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }) => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
