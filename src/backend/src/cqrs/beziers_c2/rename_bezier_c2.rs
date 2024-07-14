use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::beziers_c2::bezier_c2_renamed::BezierC2Renamed;

pub struct RenameBezierC2 {
    pub id: u64,
    pub name: String,
}

impl Command<RenameBezierC2> for RenameBezierC2 {
    fn execute(command: &RenameBezierC2, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier_c2 = backend.storage.beziers_c2.get_mut(&command.id).unwrap();
        bezier_c2.rename(&command.name);
        let bezier_renamed = Rc::new(BezierC2Renamed::new(&bezier_c2));
        drop(backend);
        let backend = app_state.borrow();
        backend.services.event_publisher.publish(bezier_renamed);
    }
}
