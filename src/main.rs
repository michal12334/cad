#[macro_use]
extern crate glium;

use std::ops::DerefMut;
use egui::Color32;
use glium::{DrawParameters, Surface};
use glium::vertex::MultiVerticesSource;
use nalgebra::{Matrix4, Point3, Vector3};
use winit::{event, event_loop};
use winit::event::MouseButton;
use backend::app_state::AppState;
use backend::cqrs::cqrs::CQRS;
use user_interface::ui::Ui;
use backend::domain::*;
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
    
    let mut app_state = AppState::new();
    
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

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    let mut ui = Ui::new();
    
    let mut mouse_position = (0.0, 0.0);
    let mut camera_position = [0.0f32, 0.0, 4.0];
    let mut view_matrix = get_view_matrix(&camera_position, &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);
    let mut mouse_middle_button_pressed = false;
    
    let color = Color32::WHITE.to_normalized_gamma_f32();
    let selected_color = Color32::YELLOW.to_normalized_gamma_f32();
    
    event_loop.run(move |event, _window_target, control_flow| {
        let mut redraw = || {
            let mut cqrs = CQRS::new(&mut app_state);
            let repaint_after = egui_glium.run(&window, ui.build(&mut cqrs));

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
                let perspective = {
                    let aspect_ratio = height as f32 / width as f32;

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
                drawing_parameters.polygon_mode = glium::draw_parameters::PolygonMode::Line;
                drawing_parameters.depth = glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                };

                target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                for torus in app_state.storage.toruses.iter() {
                    let vertex_buffer = glium::VertexBuffer::new(&display, &torus.1.mesh.vertices).unwrap();
                    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::LinesList, &torus.1.mesh.indices).unwrap();
                    let model_matrix = torus.1.transformer.get_model_matrix();
                    let color = if app_state.storage.selected_objects.iter().any(|so| so.torus_id == *torus.0) { selected_color } else { color };
                    target.draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniform! {
                            perspective: perspective,
                            model_matrix: model_matrix,
                            view: view_matrix,
                            obj_color: color 
                        },
                        &drawing_parameters)
                        .unwrap();
                }

                egui_glium.paint(&display, &mut target);
                
                target.finish().unwrap();
            }
        };
        match event {
            event::Event::RedrawRequested(_) => redraw(),

            event::Event::WindowEvent { event, .. } => {
                use event::WindowEvent;
                match &event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        *control_flow = event_loop::ControlFlow::Exit;
                    }
                    WindowEvent::Resized(new_size) => {
                        display.resize((*new_size).into());
                        width = new_size.width;
                        height = new_size.height;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let delta = (position.x - mouse_position.0, position.y - mouse_position.1);
                        mouse_position = (position.x, position.y);
                        if mouse_middle_button_pressed {
                            camera_position[0] += delta.0 as f32 / width as f32 * 5.0;
                            camera_position[1] += delta.1 as f32 / height as f32 * 5.0;
                            view_matrix = get_view_matrix(&camera_position, &[-camera_position[0], -camera_position[1], -camera_position[2]], &[0.0, 1.0, 0.0]);
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if *button == MouseButton::Middle { 
                            mouse_middle_button_pressed = *state == event::ElementState::Pressed;
                        } 
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        match delta {
                            event::MouseScrollDelta::LineDelta(_x, y) => {
                                camera_position[2] += -*y / 10.0;
                                view_matrix = get_view_matrix(&camera_position, &[-camera_position[0], -camera_position[1], -camera_position[2]], &[0.0, 1.0, 0.0]);
                            }
                            _ => {}
                        }
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

fn get_view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
