use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::cqrs::surfaces_c0::create_surface_c0::CreateSurfaceInfoDTO;
use crate::domain::events::points::point_created::PointCreated;
use crate::domain::events::surfaces_c2::surface_c2_created::SurfaceC2Created;
use crate::services::create_surface::create_surface_c2;

pub struct CreateSurfaceC2 {
    pub id: u64,
    pub create_surface_info: CreateSurfaceInfoDTO,
}

impl Command<CreateSurfaceC2> for CreateSurfaceC2 {
    fn execute(command: &CreateSurfaceC2, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let cursor_position = backend.storage.cursor.transformer.clone();
        let id_generator = &mut backend.services.id_generator;
        let (surface, points) = create_surface_c2(
            command.id,
            &command.create_surface_info,
            id_generator,
            &cursor_position,
        );
        backend.storage.surfaces_c2.insert(command.id, surface);
        let events = points
            .iter()
            .map(|point| Rc::new(PointCreated::new(point.id, point.name.clone())))
            .collect::<Vec<_>>();
        for point in points {
            backend.storage.points.insert(point.id, point);
        }
        drop(backend);
        let backend = app_state.borrow();
        for event in events {
            backend.services.event_publisher.publish(event);
        }
        backend
            .services
            .event_publisher
            .publish(Rc::new(SurfaceC2Created::new(
                command.id,
                command.create_surface_info.size,
                command.create_surface_info.is_cylinder,
            )));
    }
}
