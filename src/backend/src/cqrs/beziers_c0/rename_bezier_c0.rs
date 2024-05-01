use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;

pub struct RenameBezierC0 {
    pub id: u64,
    pub name: String,
}

impl Command<RenameBezierC0> for RenameBezierC0 {
    fn execute(command: &RenameBezierC0, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let bezier_c0 = app_state.storage.beziers_c0.get_mut(&command.id).unwrap();
        bezier_c0.rename(&command.name);
    }
}
