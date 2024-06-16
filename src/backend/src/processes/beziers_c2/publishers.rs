use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::bezier_c2_created::BezierC2Created;

pub struct BezierC2CreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<BezierC2Created> for BezierC2CreatedPublisher {
    fn consume(&self, event: &BezierC2Created) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::bezier_c2_created::BezierC2Created::new(
            event.id,
        ));
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for BezierC2CreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}