use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::bezier_c0_deleted::BezierC0Deleted;
use crate::domain::events::bezier_c2_deleted::BezierC2Deleted;
use crate::domain::events::bezier_int_deleted::BezierIntDeleted;

pub struct DeleteSelectedObjects;

impl Command<DeleteSelectedObjects> for DeleteSelectedObjects {
    fn execute(_command: &DeleteSelectedObjects, app_state: Rc<RefCell<Backend>>) {
        let mut binding = app_state.borrow_mut();
        let backend = binding.deref_mut();
        backend.storage.toruses.retain(|_, torus| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.torus_id == Some(torus.id))
        });
        backend.storage.beziers_c0.retain(|_, bezier| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.bezier_c0_id == Some(bezier.id))
        });
        backend.storage.beziers_c2.retain(|_, bezier| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.bezier_c2_id == Some(bezier.id))
        });
        backend.storage.beziers_int.retain(|_, bezier| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.bezier_int_id == Some(bezier.id))
        });
        backend.storage.points.retain(|_, point| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.point_id == Some(point.id))
                || backend
                    .storage
                    .beziers_c0
                    .values()
                    .any(|b| b.points.iter().any(|p| p.id == point.id))
                || backend
                    .storage
                    .beziers_c2
                    .values()
                    .any(|b| b.b_spline_points.iter().any(|p| p.id == point.id))
                || backend
                    .storage
                    .beziers_int
                    .values()
                    .any(|b| b.points.iter().any(|p| p.id == point.id))
        });

        let deleted_beziers_c0 = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.bezier_c0_id)
            .collect::<Vec<_>>();

        let deleted_beziers_c2 = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.bezier_c2_id)
            .collect::<Vec<_>>();

        let deleted_beziers_int = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.bezier_int_id)
            .collect::<Vec<_>>();

        backend.storage.selected_objects.clear();

        drop(binding);

        let backend = app_state.borrow();
        deleted_beziers_c0.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(BezierC0Deleted::new(*id)));
        });
        deleted_beziers_c2.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(BezierC2Deleted::new(*id)));
        });
        deleted_beziers_int.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(BezierIntDeleted::new(*id)));
        });
    }
}
