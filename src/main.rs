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
            gl_Position = perspective * model_matrix * vec4(position, 1.0);
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
    let mut camera_position = Point3::new(0.0f32, 0.0, 4.0);
    let mut view_matrix = Matrix4::face_towards(&camera_position, &Point3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 1.0, 0.0));
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
                let view_matrix = *view_matrix.as_ref();
                
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
                            camera_position = camera_position + Vector3::new(-delta.0 as f32 / width as f32, delta.1 as f32 / height as f32, 0.0);
                            view_matrix = Matrix4::face_towards(&camera_position, &Point3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 1.0, 0.0));
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
                                camera_position = camera_position + Vector3::new(0.0, 0.0, -*y / 10.0);
                                view_matrix = Matrix4::face_towards(&camera_position, &Point3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 1.0, 0.0));
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
