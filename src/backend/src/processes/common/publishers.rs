use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::common::scene_loaded::SceneLoaded;

pub struct SceneLoadedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<SceneLoaded> for SceneLoadedPublisher {
    fn consume(&self, message: &SceneLoaded) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::common::scene_loaded::SceneLoaded {});
        backend.services.event_publisher.publish(event);
    }
}
impl AnyConsumer for SceneLoadedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

