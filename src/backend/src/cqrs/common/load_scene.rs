use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::common::scene_loaded::SceneLoaded;
use crate::services::file_helpers::load_scene::load_scene;

pub struct LoadScene {
    pub file_path: String,
}

impl Command<LoadScene> for LoadScene {
    fn execute(command: &LoadScene, app_state: Rc<RefCell<Backend>>) {
        load_scene(&mut app_state.borrow_mut().storage, &command.file_path);
        let backend = app_state.borrow();
        let max_id = backend
            .storage
            .points
            .keys()
            .max()
            .max(backend.storage.toruses.keys().max())
            .max(backend.storage.beziers_c0.keys().max())
            .max(backend.storage.beziers_c2.keys().max())
            .max(backend.storage.beziers_int.keys().max())
            .max(backend.storage.surfaces_c0.keys().max())
            .max(backend.storage.surfaces_c2.keys().max());
        let next_id = max_id.map(|id| id + 1).unwrap_or(1);
        drop(backend);
        app_state
            .borrow_mut()
            .services
            .id_generator
            .set_next_id(next_id);
        app_state
            .borrow()
            .services
            .event_publisher
            .publish(Rc::new(SceneLoaded {}));
    }
}
