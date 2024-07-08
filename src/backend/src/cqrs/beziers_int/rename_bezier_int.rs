use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;

pub struct RenameBezierInt {
    pub id: u64,
    pub name: String,
}

impl Command<RenameBezierInt> for RenameBezierInt {
    fn execute(command: &RenameBezierInt, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier_int = backend.storage.beziers_int.get_mut(&command.id).unwrap();
        bezier_int.rename(&command.name);
    }
}
