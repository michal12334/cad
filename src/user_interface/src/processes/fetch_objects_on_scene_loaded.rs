use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend::cqrs::cqrs::CQRS;
use backend_events::common::scene_loaded::SceneLoaded;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::ui::Ui;

pub struct FetchObjectsOnSceneLoaded {
    pub ui: Rc<RefCell<Ui>>,
    pub cqrs: CQRS,
}

impl Consumer<SceneLoaded> for FetchObjectsOnSceneLoaded {
    fn consume(&self, event: &SceneLoaded) {
        self.ui.borrow_mut().fetch_objects(&self.cqrs);
    }
}

impl AnyConsumer for FetchObjectsOnSceneLoaded {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
