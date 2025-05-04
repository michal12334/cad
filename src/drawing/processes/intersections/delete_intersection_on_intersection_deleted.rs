use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::intersections::intersection_deleted::IntersectionDeleted;

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct DeleteIntersectionOnIntersectionDeleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<IntersectionDeleted> for DeleteIntersectionOnIntersectionDeleted {
    fn consume(&self, event: &IntersectionDeleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.intersections.remove(&event.id);
    }
}

impl AnyConsumer for DeleteIntersectionOnIntersectionDeleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
