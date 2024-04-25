use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;

pub struct UpdateTorus {
    pub id: u64,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
}

impl Command<UpdateTorus> for UpdateTorus {
    fn execute(command: &UpdateTorus, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        if command.minor_radius >= command.major_radius {
            return;
        }

        let torus = app_state.storage.toruses.get_mut(&command.id).unwrap();
        torus.update(
            command.major_radius,
            command.minor_radius,
            command.major_segments,
            command.minor_segments,
        );
    }
}
