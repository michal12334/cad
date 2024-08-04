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
        app_state.borrow().services.event_publisher.publish(Rc::new(SceneLoaded {}));
    }
}
