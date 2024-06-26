#[macro_use]
extern crate glium;
extern crate user_interface;

use std::cell::RefCell;
use std::rc::Rc;

use egui::Color32;
use glium::Surface;
use winit::event::ElementState::Pressed;
use winit::event::MouseButton;
use winit::{event, event_loop};

use backend::backend::Backend;
use backend::cqrs::common::selected_objects_center::SelectedObjectsCenter;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::cursors::transform_cursor::TransformCursor;
use backend::cqrs::points::all_points::AllPoints;
use backend::cqrs::points::point_details::LittleTransformerDTO;
use backend::domain::point::Point;
use backend::domain::transformer::LittleTransformer;
use backend::processes::beziers_c0::add_point_to_selected_beziers_c0_on_point_created::AddPointToSelectedBeziersC0OnPointCreated;
use backend::processes::beziers_c0::publishers::{
    BezierC0PointsDeletedPublisher, BezierC0RenamedPublisher, PointAddedToBezierC0Publisher,
};
use infrastructure::event_bus::EventBus;
use math::vector4::Vector4;
use user_interface::processes::sync_bezier_c0_with_backend::{
    SyncBezierC0AddedPointsWithBackend, SyncBezierC0DeletedPointsWithBackend,
    SyncBezierC0NameWithBackend,
};
use user_interface::ui::Ui;

use crate::bezier_c0_drawer::BezierC0Drawer;
use crate::infinite_grid_drawer::InfiniteGridDrawer;
use crate::point_drawer::PointDrawer;
use crate::polygon_drawer::PolygonDrawer;
use crate::torus_drawer::TorusDrawer;

mod bezier_c0_drawer;
mod cursor_drawer;
mod infinite_grid_drawer;
mod point_drawer;
mod polygon_drawer;
mod torus_drawer;

fn main() {
    let mut width = 800;
    let mut height = 600;

    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("CAD")
        .with_inner_size(width, height)
        .build(&event_loop);

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &window, &event_loop);

    let event_bus = EventBus::new();
    let event_bus = Rc::new(RefCell::new(event_bus));

    let app_state = Rc::new(RefCell::new(Backend::new(event_bus.clone())));
    let ui = Rc::new(RefCell::new(Ui::new()));

    event_bus
        .borrow_mut()
        .add_consumer(BezierC0RenamedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC0PointsDeletedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(PointAddedToBezierC0Publisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddPointToSelectedBeziersC0OnPointCreated {
            backend: app_state.clone(),
        });

    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierC0NameWithBackend { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierC0DeletedPointsWithBackend { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierC0AddedPointsWithBackend { ui: ui.clone() });

    let torus_drawer = TorusDrawer::new(&display);
    let point_drawer = PointDrawer::new(&display);
    let cursor_drawer = cursor_drawer::CursorDrawer::new(&display);
    let infinite_grid_drawer = InfiniteGridDrawer::new(&display);
    let bezier_c0_drawer = BezierC0Drawer::new(&display);
    let polygon_drawer = PolygonDrawer::new(&display);

    let mut mouse_position = (0.0, 0.0);
    let mut camera_direction = math::vector3::Vector3::new(0.0f32, 0.0, 1.0);
    let mut camera_angle = math::vector3::Vector3::new(0.0f32, 0.0, 0.0);
    let mut camera_up = math::vector3::Vector3::new(0.0f32, 1.0, 0.0);
    let mut camera_distant = 4.0f32;
    let mut view_matrix = math::matrix4::Matrix4::view(
        camera_direction * camera_distant * (-1.0),
        camera_direction,
        camera_up,
    );
    let mut mouse_middle_button_pressed = false;

    let color = Color32::WHITE.to_normalized_gamma_f32();
    let selected_color = Color32::YELLOW.to_normalized_gamma_f32();

    event_loop.run(move |event, _window_target, control_flow| {
        let mut redraw = || {
            let mut cqrs = CQRS::new(app_state.clone());
            let repaint_after;
            unsafe {
                let ui = ui.as_ptr();
                repaint_after = egui_glium.run(&window, (*ui).build(&mut cqrs));
            }

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

                let cqrs = CQRS::new(app_state.clone());

                let app_state = app_state.borrow();

                for torus in app_state.storage.toruses.iter() {
                    let color = if app_state.storage.selected_objects.iter().any(|so| so.torus_id == Some(*torus.0)) { selected_color } else { color };
                    torus_drawer.draw(&mut target, &display, &torus.1, &perspective, &view_matrix, color);
                }

                for point in app_state.storage.points.iter() {
                    let color = if app_state.storage.selected_objects.iter().any(|so| so.point_id == Some(*point.0)) { selected_color } else { color };
                    point_drawer.draw(&mut target, &display, &point.1, &perspective, &view_matrix, color);
                }

                let center_point = cqrs.get(&SelectedObjectsCenter);
                if let Some(center_point) = center_point {
                    let mut transformer = LittleTransformer::new();
                    transformer.position = center_point.position;
                    point_drawer.draw(&mut target, &display, &Point::new(0, transformer), &perspective, &view_matrix, Color32::BROWN.to_normalized_gamma_f32());
                }

                for bezier_points in app_state.storage.beziers_c0.values().map(|b| b.points.iter().map(|p| app_state.storage.points.get(&p.id).unwrap().clone()).collect::<Vec<Point>>()) {
                    bezier_c0_drawer.draw(&mut target, &display, &bezier_points, &perspective, &view_matrix, color, width, height);
                }

                for bezier_points in app_state.storage.beziers_c0.values().filter(|b| b.draw_polygon).map(|b| b.points.iter().map(|p| app_state.storage.points.get(&p.id).unwrap().clone()).collect::<Vec<Point>>()) {
                    polygon_drawer.draw(&mut target, &display, &bezier_points, &perspective, &view_matrix, color);
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
                            camera_direction = (math::matrix4::Matrix4::rotation_y(camera_angle.y) * math::matrix4::Matrix4::rotation_x(camera_angle.x) * Vector4::new(0.0, 0.0, 1.0, 0.0)).xyz();
                            camera_up = (math::matrix4::Matrix4::rotation_y(camera_angle.y) * math::matrix4::Matrix4::rotation_x(camera_angle.x) * Vector4::new(0.0, 1.0, 0.0, 0.0)).xyz();
                            view_matrix = math::matrix4::Matrix4::view(camera_direction * camera_distant * (-1.0), camera_direction, camera_up);
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if *button == MouseButton::Middle {
                            mouse_middle_button_pressed = *state == Pressed;
                        } else if *button == MouseButton::Left && !ui.borrow().is_pointer_over_area() && *state == Pressed {
                            let x = mouse_position.0 / width as f64 * 2.0 - 1.0;
                            let y = 1.0 - mouse_position.1 / height as f64 * 2.0;
                            let point = Vector4::new(x as f32, y as f32, 0.95, 1.0);
                            let inversed_view_matrix = view_matrix.get_inversed();
                            let inversed_perspective_matrix = math::matrix4::Matrix4::perspective(std::f32::consts::PI / 3.0, width as f32 / height as f32, 0.1, 1024.0).get_inversed();
                            let point = (inversed_view_matrix * inversed_perspective_matrix * point).to_vector3();
                            let mut cqrs = CQRS::new(app_state.clone());
                            cqrs.execute(&TransformCursor { transformer: LittleTransformerDTO { position: (point.x as f64, point.y as f64, point.z as f64) } });
                        } else if *button == MouseButton::Right && !ui.borrow().is_pointer_over_area() && *state == Pressed {
                            let mut cqrs = CQRS::new(app_state.clone());
                            let points = cqrs.get(&AllPoints);
                            for point in points {
                                let position = Vector4::new(point.transformer.position.0 as f32, point.transformer.position.1 as f32, point.transformer.position.2 as f32, 1.0);
                                let position = (math::matrix4::Matrix4::perspective(std::f32::consts::PI / 3.0, width as f32 / height as f32, 0.1, 1024.0) * view_matrix * position).to_vector3();
                                let x = mouse_position.0 as f32 / width as f32 * 2.0 - 1.0;
                                let y = 1.0 - mouse_position.1 as f32 / height as f32 * 2.0;
                                if (position.x - x) * (position.x - x) + (position.y - y) * (position.y - y) <= 0.005 {
                                    ui.borrow_mut().change_point_selection(point.id, &mut cqrs);
                                }
                            }
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        if !ui.borrow().is_pointer_over_area() {
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
                            ui.borrow_mut().set_control_pressed(input.state == Pressed);
                        } else if input.virtual_keycode == Some(event::VirtualKeyCode::Delete) && input.state == Pressed {
                            let mut cqrs = CQRS::new(app_state.clone());
                            cqrs.execute(&backend::cqrs::common::delete_selected_objects::DeleteSelectedObjects);
                            ui.borrow_mut().fetch_objects(&mut cqrs);
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
