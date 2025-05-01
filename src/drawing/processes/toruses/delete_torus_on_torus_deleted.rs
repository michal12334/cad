use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::toruses::torus_deleted::TorusDeleted;

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct DeleteTorusOnTorusDeleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<TorusDeleted> for DeleteTorusOnTorusDeleted {
    fn consume(&self, event: &TorusDeleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.toruses.remove(&event.id);
    }
}

impl AnyConsumer for DeleteTorusOnTorusDeleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
