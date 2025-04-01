use std::{cell::RefCell, rc::Rc};

use crate::{
    backend::Backend, cqrs::cqrs::Command,
    domain::events::gregories::gregory_renamed::GregoryRenamed,
};

pub struct RenameGregory {
    pub id: u64,
    pub name: String,
}

impl Command<RenameGregory> for RenameGregory {
    fn execute(command: &RenameGregory, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let gregory = backend.storage.gregories.get_mut(&command.id).unwrap();
        gregory.rename(&command.name);
        let gregory_renamed = Rc::new(GregoryRenamed::new(gregory.id, gregory.name.clone()));
        drop(backend);
        let backend = app_state.borrow();
        backend.services.event_publisher.publish(gregory_renamed);
    }
}
