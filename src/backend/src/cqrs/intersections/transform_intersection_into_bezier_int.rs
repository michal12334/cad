use std::{cell::RefCell, rc::Rc};

use crate::{
    backend::Backend,
    cqrs::cqrs::Command,
    domain::{
        bezier_int::BezierInt,
        events::{
            beziers_int::bezier_int_created::BezierIntCreated,
            intersections::intersection_deleted::IntersectionDeleted,
            points::point_created::PointCreated,
        },
        point::Point,
        transformer::LittleTransformer,
    },
    extensions::iterator_extensions::IteratorExtensions,
};

pub struct TransformIntersectionIntoBezierInt {
    pub id: u64,
}

impl Command<TransformIntersectionIntoBezierInt> for TransformIntersectionIntoBezierInt {
    fn execute(command: &TransformIntersectionIntoBezierInt, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let points = backend.storage.intersections[&command.id]
            .intersection_points
            .clone();
        let wrap = backend.storage.intersections[&command.id].wrap;
        let id_generator = &mut backend.services.id_generator;
        let points = points
            .iter()
            .map(|p| {
                Point::new(
                    id_generator.next(),
                    LittleTransformer {
                        position: (p.x as f64, p.y as f64, p.z as f64),
                    },
                )
            })
            .collect::<Vec<_>>();
        let bezier_int = BezierInt::new(
            id_generator.next(),
            points
                .iter()
                .cloned()
                .chain_if([points[0].clone()].into_iter(), wrap)
                .collect(),
        );
        let point_created_events = points
            .iter()
            .map(|p| PointCreated::new(p.id, p.name.clone()))
            .collect::<Vec<_>>();
        let intersection_deleted_event = IntersectionDeleted::new(command.id);
        let bezier_int_created_event = BezierIntCreated::new(bezier_int.id);
        backend
            .storage
            .points
            .extend(points.iter().map(|p| (p.id, p.clone())));
        backend
            .storage
            .beziers_int
            .insert(bezier_int.id, bezier_int);
        backend.storage.intersections.remove(&command.id);
        drop(backend);
        let backend = app_state.borrow();
        backend
            .services
            .event_publisher
            .publish(Rc::new(bezier_int_created_event));
        for event in point_created_events {
            backend.services.event_publisher.publish(Rc::new(event));
        }
        backend
            .services
            .event_publisher
            .publish(Rc::new(intersection_deleted_event));
    }
}
