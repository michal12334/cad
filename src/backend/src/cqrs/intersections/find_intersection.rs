use std::{cell::RefCell, rc::Rc};

use math::vector3::Vector3;

use crate::{
    backend::Backend,
    cqrs::cqrs::Command,
    domain::{
        events::intersections::intersection_created::IntersectionCreated,
        intersection::Intersection, intersection_object::IntersectionObject,
    },
};

pub struct FindIntersection {
    pub id1: IntersectionObjectIdDTO,
    pub id2: IntersectionObjectIdDTO,
    pub intersection_id: u64,
    pub texture_size: usize,
    pub newton_factor: f32,
    pub rough: bool,
    pub max_distance: f32,
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

        let intersection_object1 = get_intersection_object(&command.id1, &mut backend);
        let intersection_object2 = get_intersection_object(&command.id2, &mut backend);

        let cursor_position = backend.storage.cursor.transformer.position;
        let cursor_position = Vector3::new(
            cursor_position.0 as f32,
            cursor_position.1 as f32,
            cursor_position.2 as f32,
        );

        let intersection = Intersection::from_objects(
            command.intersection_id,
            format!(
                "Intersection {}-{}",
                intersection_object1.id, intersection_object2.id
            ),
            intersection_object1.id.clone(),
            intersection_object2.id.clone(),
            &intersection_object1,
            &intersection_object2,
            &cursor_position,
            command.texture_size,
            command.newton_factor,
            command.rough,
            command.max_distance,
        );

        if intersection.is_none() {
            return;
        }
        let intersection = intersection.unwrap();

        let event = IntersectionCreated::new(
            intersection.id,
            intersection.name.clone(),
            intersection.uv_texture.clone(),
            intersection.st_texture.clone(),
            intersection.intersection_points.clone(),
            intersection.wrap,
        );

        backend
            .storage
            .intersections
            .insert(command.intersection_id, intersection);

        drop(backend);

        let backend = app_state.borrow();

        backend.services.event_publisher.publish(Rc::new(event));
    }
}

fn get_intersection_object(
    id: &IntersectionObjectIdDTO,
    backend: &mut Backend,
) -> IntersectionObject {
    match id {
        IntersectionObjectIdDTO::Torus(id) => {
            let torus = backend.storage.toruses.get(id).unwrap();
            torus.get_intersection_object()
        }
        IntersectionObjectIdDTO::SurfaceC0(id) => {
            let surface = backend.storage.surfaces_c0.get(id).unwrap();
            let points = backend.storage.points.values().cloned().collect::<Vec<_>>();
            surface.get_intersection_object(&points)
        }
        IntersectionObjectIdDTO::SurfaceC2(id) => {
            let surface = backend.storage.surfaces_c2.get(id).unwrap();
            let points = backend.storage.points.values().cloned().collect::<Vec<_>>();
            surface.get_intersection_object(&points)
        }
    }
}
