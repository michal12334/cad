use std::{cell::RefCell, rc::Rc};

use crate::{
    backend::Backend, cqrs::cqrs::Command,
    domain::events::gregories::gregory_settings_updated::GregorySettingsUpdated,
};

pub struct UpdateGregorySettings {
    pub id: u64,
    pub tess_level: u8,
    pub draw_vectors: bool,
}

impl Command<UpdateGregorySettings> for UpdateGregorySettings {
    fn execute(command: &UpdateGregorySettings, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let gregory = backend.storage.gregories.get_mut(&command.id).unwrap();
        gregory.update_settings(command.tess_level, command.draw_vectors);
        let gregory_settings_updated = Rc::new(GregorySettingsUpdated::new(
            gregory.id,
            gregory.tess_level,
            gregory.draw_vectors,
        ));
        drop(backend);
        let backend = app_state.borrow();
        backend
            .services
            .event_publisher
            .publish(gregory_settings_updated);
    }
}
