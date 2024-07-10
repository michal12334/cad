use crate::drawing::drawing_storage::DrawingStorage;
use backend_events::bezier_int_deleted::BezierIntDeleted;
use infrastructure::consumer::{AnyConsumer, Consumer};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DeleteBezierIntOnBezierIntDeleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<BezierIntDeleted> for DeleteBezierIntOnBezierIntDeleted {
    fn consume(&self, event: &BezierIntDeleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.beziers_int.remove(&event.id);
    }
}

impl AnyConsumer for DeleteBezierIntOnBezierIntDeleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
