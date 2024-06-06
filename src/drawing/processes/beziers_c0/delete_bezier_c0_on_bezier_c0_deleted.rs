use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend_events::bezier_c0_deleted::BezierC0Deleted;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::drawing_storage::DrawingStorage;

pub struct DeleteBezierC0OnBezierC0Deleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<BezierC0Deleted> for DeleteBezierC0OnBezierC0Deleted {
    fn consume(&self, event: &BezierC0Deleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.beziers_c0.remove(&event.id);
    }
}

impl AnyConsumer for DeleteBezierC0OnBezierC0Deleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
