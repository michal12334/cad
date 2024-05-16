use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;

pub struct DeleteSelectedObjects;

impl Command<DeleteSelectedObjects> for DeleteSelectedObjects {
    fn execute(_command: &DeleteSelectedObjects, app_state: Rc<RefCell<Backend>>) {
        let mut binding = app_state.borrow_mut();
        let app_state = binding.deref_mut();
        app_state.storage.toruses.retain(|_, torus| {
            !app_state
                .storage
                .selected_objects
                .iter()
                .any(|object| object.torus_id == Some(torus.id))
        });
        app_state.storage.beziers_c0.retain(|_, bezier| {
            !app_state
                .storage
                .selected_objects
                .iter()
                .any(|object| object.bezier_c0_id == Some(bezier.id))
        });
        app_state.storage.points.retain(|_, point| {
            !app_state
                .storage
                .selected_objects
                .iter()
                .any(|object| object.point_id == Some(point.id))
                || app_state
                    .storage
                    .beziers_c0
                    .values()
                    .any(|b| b.points.iter().any(|p| p.id == point.id))
        });
        app_state.storage.selected_objects.clear();
    }
}
