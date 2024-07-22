use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::points::point_created::PointCreated;
use crate::domain::events::surfaces_c0::surface_c0_created::SurfaceC0Created;
use crate::services::create_surface::create_surface;

pub struct CreateSurface {
    pub id: u64,
    pub create_surface_info: CreateSurfaceInfoDTO
}

pub struct CreateSurfaceInfoDTO {
    pub is_cylinder: bool,
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub radius: Option<f64>,
    pub size: (u32, u32),
}

impl Command<CreateSurface> for CreateSurface {
    fn execute(command: &CreateSurface, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let cursor_position = backend.storage.cursor.transformer.clone();
        let id_generator = &mut backend.services.id_generator;
        let (surface, points) = create_surface(command.id, &command.create_surface_info, id_generator, &cursor_position);
        backend.storage.surfaces_c0.insert(command.id, surface);
        let events = points.iter().map(|point| Rc::new(PointCreated::new(point.id, point.name.clone()))).collect::<Vec<_>>();
        for point in points {
            backend.storage.points.insert(point.id, point);
        }
        drop(backend);
        let backend = app_state.borrow();
        for event in events {
            backend.services.event_publisher.publish(event);
        }
        backend.services.event_publisher.publish(Rc::new(SurfaceC0Created::new(command.id, command.create_surface_info.size)));
    }
}
