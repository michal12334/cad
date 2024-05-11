use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::bezier_c0_renamed::BezierC0Renamed;

pub struct RenameBezierC0 {
    pub id: u64,
    pub name: String,
}

impl Command<RenameBezierC0> for RenameBezierC0 {
    fn execute(command: &RenameBezierC0, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier_c0 = backend.storage.beziers_c0.get_mut(&command.id).unwrap();
        bezier_c0.rename(&command.name);
        let bezier_renamed = Rc::new(BezierC0Renamed::new(&bezier_c0));
        drop(backend);
        let backend = app_state.borrow();
        backend.services.event_publisher.publish(bezier_renamed);
    }
}
