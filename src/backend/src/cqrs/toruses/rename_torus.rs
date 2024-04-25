use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;

pub struct RenameTorus {
    pub id: u64,
    pub name: String,
}

impl Command<RenameTorus> for RenameTorus {
    fn execute(command: &RenameTorus, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let torus = app_state.storage.toruses.get_mut(&command.id).unwrap();
        torus.rename(&command.name);
    }
}
