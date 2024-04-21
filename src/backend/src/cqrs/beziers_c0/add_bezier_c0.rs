use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::bezier_c0::{BezierC0, BezierC0Point};
use std::cell::RefCell;
use std::rc::Rc;

pub struct AddBezierC0 {
    pub id: u64,
}

impl Command<AddBezierC0> for AddBezierC0 {
    fn execute(command: &AddBezierC0, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let bezier = BezierC0::new(
            command.id,
            app_state
                .storage
                .selected_objects
                .iter()
                .flat_map(|object| {
                    app_state
                        .storage
                        .points
                        .values()
                        .filter(|point| object.point_id == Some(point.id))
                        .map(|point| BezierC0Point { id: point.id })
                })
                .collect(),
        );
        app_state.storage.beziers_c0.insert(command.id, bezier);
    }
}
