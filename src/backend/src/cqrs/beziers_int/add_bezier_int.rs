use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::bezier_int::BezierInt;
use crate::domain::events::bezier_int_created::BezierIntCreated;

pub struct AddBezierInt {
    pub id: u64,
}

impl Command<AddBezierInt> for AddBezierInt {
    fn execute(command: &AddBezierInt, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = BezierInt::new(
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
        backend.storage.beziers_int.insert(command.id, bezier);
        drop(backend);
        let backend = app_state.borrow();
        backend
            .services
            .event_publisher
            .publish(Rc::new(BezierIntCreated::new(command.id)));
    }
}
