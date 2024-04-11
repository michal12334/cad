use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use std::cell::RefCell;
use std::rc::Rc;

pub struct RenamePoint {
    pub id: u64,
    pub name: String,
}

impl Command<RenamePoint> for RenamePoint {
    fn execute(command: &RenamePoint, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let point = app_state.storage.points.get_mut(&command.id).unwrap();
        point.rename(&command.name);
    }
}
