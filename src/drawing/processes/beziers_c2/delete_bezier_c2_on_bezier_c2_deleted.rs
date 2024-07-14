use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::beziers_c2::bezier_c2_deleted::BezierC2Deleted;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct DeleteBezierC2OnBezierC2Deleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<BezierC2Deleted> for DeleteBezierC2OnBezierC2Deleted {
    fn consume(&self, event: &BezierC2Deleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.beziers_c2.remove(&event.id);
    }
}

impl AnyConsumer for DeleteBezierC2OnBezierC2Deleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
