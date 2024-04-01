mod infinite_grid_drawer;
mod torus_drawer;
mod point_drawer;
mod cursor_drawer;

#[macro_use]
extern crate glium;

use std::ops::DerefMut;
use egui::Color32;
use glium::{DrawParameters, Surface};
use glium::vertex::MultiVerticesSource;
use winit::{event, event_loop};
use winit::event::MouseButton;
use backend::app_state::AppState;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::cursors::transform_cursor::TransformCursor;
use backend::cqrs::points::point_details::LittleTransformerDTO;
use user_interface::ui::Ui;
use backend::domain::*;
use math::vector4::Vector4;
use crate::infinite_grid_drawer::InfiniteGridDrawer;
use crate::point_drawer::PointDrawer;
use crate::torus_drawer::TorusDrawer;

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
    
    let torus_drawer = TorusDrawer::new(&display);
    let point_drawer = PointDrawer::new(&display);
    let cursor_drawer = cursor_drawer::CursorDrawer::new(&display);
    let infinite_grid_drawer = InfiniteGridDrawer::new(&display);
    
    let mut ui = Ui::new();
    
    let mut mouse_position = (0.0, 0.0);
    let mut camera_direction = math::vector3::Vector3::new(0.0f32, 0.0, 1.0);
    let mut camera_angle = math::vector3::Vector3::new(0.0f32, 0.0, 0.0);
    let mut camera_up = math::vector3::Vector3::new(0.0f32, 1.0, 0.0);
    let mut camera_distant = 4.0f32;
    let mut view_matrix = math::matrix4::Matrix4::view(camera_direction * camera_distant * (-1.0), camera_direction, camera_up);
    let mut mouse_middle_button_pressed = false;
    let mut control_pressed = false;
    
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
                let perspective = math::matrix4::Matrix4::perspective(std::f32::consts::PI / 3.0, width as f32 / height as f32, 0.1, 1024.0);
                
                let mut target = display.draw();

                target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                for torus in app_state.storage.toruses.iter() {
                    let color = if app_state.storage.selected_objects.iter().any(|so| so.torus_id == Some(*torus.0)) { selected_color } else { color };
                    torus_drawer.draw(&mut target, &display, &torus.1, &perspective, &view_matrix, color);
                }

                for point in app_state.storage.points.iter() {
                    let color = if app_state.storage.selected_objects.iter().any(|so| so.point_id == Some(*point.0)) { selected_color } else { color };
                    point_drawer.draw(&mut target, &display, &point.1, &perspective, &view_matrix, color);
                }
                
                cursor_drawer.draw(&mut target, &display, &app_state.storage.cursor, &perspective, &view_matrix);
                
                infinite_grid_drawer.draw(&mut target, &perspective.data, &view_matrix.data);

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
                            camera_angle.x += delta.1 as f32 * 0.01;
                            camera_angle.y += delta.0 as f32 * 0.01 * if camera_angle.x.cos() < 0.0 { -1.0 } else { 1.0 };
                            camera_direction = (Vector4::new(0.0, 0.0, 1.0, 0.0) * math::matrix4::Matrix4::rotation_x(camera_angle.x) * math::matrix4::Matrix4::rotation_y(camera_angle.y)).xyz();
                            camera_up = (Vector4::new(0.0, 1.0, 0.0, 0.0) * math::matrix4::Matrix4::rotation_x(camera_angle.x) * math::matrix4::Matrix4::rotation_y(camera_angle.y)).xyz();
                            view_matrix = math::matrix4::Matrix4::view(camera_direction * camera_distant * (-1.0), camera_direction, camera_up);
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if *button == MouseButton::Middle { 
                            mouse_middle_button_pressed = *state == event::ElementState::Pressed;
                        } else if *button == MouseButton::Left && !ui.is_pointer_over_area() && *state == event::ElementState::Pressed { 
                            let x = mouse_position.0 / width as f64 * 2.0 - 1.0;
                            let y = 1.0 - mouse_position.1 / height as f64 * 2.0;
                            let point = Vector4::new(x as f32, y as f32, -1.0, 1.0);
                            let inversed_view_matrix = view_matrix.get_inversed();
                            let inversed_perspective_matrix = math::matrix4::Matrix4::perspective(std::f32::consts::PI / 3.0, width as f32 / height as f32, 0.1, 1024.0).get_inversed();
                            let point = (point * inversed_perspective_matrix * inversed_view_matrix).to_vector3();
                            let mut cqrs = CQRS::new(&mut app_state);
                            cqrs.execute(&TransformCursor { transformer: LittleTransformerDTO { position: (point.x as f64, point.y as f64, point.z as f64) } });
                        } 
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        if !ui.is_pointer_over_area() {
                            match delta {
                                event::MouseScrollDelta::LineDelta(_x, y) => {
                                    camera_distant += -y * 0.1;
                                    view_matrix = math::matrix4::Matrix4::view(camera_direction * camera_distant * (-1.0), camera_direction, camera_up);
                                }
                                _ => {}
                            }
                        }
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.virtual_keycode == Some(event::VirtualKeyCode::LControl) {
                            control_pressed = input.state == event::ElementState::Pressed;
                            ui.set_control_pressed(control_pressed);
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
