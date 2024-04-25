use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::torus::Torus;
use crate::domain::transformer::Transformer;

pub struct AddTorus {
    pub id: u64,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
}

impl Command<AddTorus> for AddTorus {
    fn execute(command: &AddTorus, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let torus = Torus::new(
            command.id,
            command.major_radius,
            command.minor_radius,
            command.major_segments,
            command.minor_segments,
            Transformer::from_cursor(&app_state.storage.cursor),
        );
        app_state.storage.toruses.insert(command.id, torus);
    }
}
