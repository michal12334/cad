use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::bezier_c0::{BezierC0, BezierC0Point};
use crate::domain::events::bezier_c0_created::BezierC0Created;

pub struct AddBezierC0 {
    pub id: u64,
}

impl Command<AddBezierC0> for AddBezierC0 {
    fn execute(command: &AddBezierC0, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = BezierC0::new(
            command.id,
            backend
                .storage
                .selected_objects
                .iter()
                .flat_map(|object| {
                    backend
                        .storage
                        .points
                        .values()
                        .filter(|point| object.point_id == Some(point.id))
                        .map(|point| BezierC0Point { id: point.id })
                })
                .collect(),
        );
        backend.storage.beziers_c0.insert(command.id, bezier);
        drop(backend);
        let backend = app_state.borrow();
        backend
            .services
            .event_publisher
            .publish(Rc::new(BezierC0Created::new(command.id)));
    }
}
