use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;

pub struct RenameSurfaceC2 {
    pub id: u64,
    pub name: String,
}

impl Command<RenameSurfaceC2> for RenameSurfaceC2 {
    fn execute(command: &RenameSurfaceC2, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let surface = backend.storage.surfaces_c2.get_mut(&command.id).unwrap();
        surface.rename(&command.name);
    }
}
