use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::bezier_c2::BezierC2;
use crate::domain::events::beziers_c2::bezier_c2_created::BezierC2Created;

pub struct AddBezierC2 {
    pub id: u64,
}

impl Command<AddBezierC2> for AddBezierC2 {
    fn execute(command: &AddBezierC2, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = BezierC2::new(
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
                        .map(|point| point.clone())
                })
                .collect(),
        );
        backend.storage.beziers_c2.insert(command.id, bezier);
        drop(backend);
        let backend = app_state.borrow();
        backend
            .services
            .event_publisher
            .publish(Rc::new(BezierC2Created::new(command.id)));
    }
}
