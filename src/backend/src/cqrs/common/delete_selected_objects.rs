use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::bezier_c0_deleted::BezierC0Deleted;

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
        });
        
        let deleted_beziers_c0 = backend.storage
            .selected_objects
            .iter()
            .filter_map(|object| object.bezier_c0_id)
            .collect::<Vec<_>>();
        
        backend.storage.selected_objects.clear();
        
        drop(binding);
        
        let backend = app_state.borrow();
        deleted_beziers_c0.iter().for_each(|id| {
            backend.services.event_publisher.publish(Rc::new(BezierC0Deleted::new(*id)));
        });
    }
}
