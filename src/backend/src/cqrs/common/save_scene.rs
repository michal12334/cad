use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::services::file_helpers::save_scene::save_scene;

pub struct SaveScene {
    pub file_path: String,
}

impl Command<SaveScene> for SaveScene {
    fn execute(command: &SaveScene, app_state: Rc<RefCell<Backend>>) {
        save_scene(&app_state.borrow().storage, &command.file_path, &mut app_state.borrow().services.id_generator.clone());
    }
}
