use std::{cell::RefCell, rc::Rc};

use math::vector3::Vector3;

use crate::{
    backend::Backend,
    cqrs::cqrs::Command,
    domain::{
        events::intersections::intersection_created::IntersectionCreated,
        intersection::{Intersection, IntersectionObjectId},
    },
};

pub struct FindIntersection {
    pub id1: IntersectionObjectIdDTO,
    pub id2: IntersectionObjectIdDTO,
    pub intersection_id: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum IntersectionObjectIdDTO {
    Torus(u64),
    SurfaceC0(u64),
    SurfaceC2(u64),
}

impl Command<FindIntersection> for FindIntersection {
    fn execute(command: &FindIntersection, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        if let IntersectionObjectIdDTO::Torus(id1) = command.id1 {
            if let IntersectionObjectIdDTO::Torus(id2) = command.id2 {
                let torus1 = &backend.storage.toruses[&id1];
                let torus2 = &backend.storage.toruses[&id2];
                let cursor_position = backend.storage.cursor.transformer.position;
                let cursor_position = Vector3::new(
                    cursor_position.0 as f32,
                    cursor_position.1 as f32,
                    cursor_position.2 as f32,
                );

                let intersection = Intersection::from_objects(
                    command.intersection_id,
                    format!("Intersection {}-{}", id1, id2),
                    IntersectionObjectId::Torus(id1),
                    IntersectionObjectId::Torus(id2),
                    &torus1.get_intersection_object(),
                    &torus2.get_intersection_object(),
                    &cursor_position,
                    200,
                );

                let event = IntersectionCreated::new(
                    intersection.id,
                    intersection.name.clone(),
                    intersection.uv_texture.clone(),
                    intersection.st_texture.clone(),
                );

                backend
                    .storage
                    .intersections
                    .insert(command.intersection_id, intersection);

                drop(backend);

                let backend = app_state.borrow();

                backend.services.event_publisher.publish(Rc::new(event));
            }
        } else {
            // Handle other types of intersection objects
        }
    }
}
