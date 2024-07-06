use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::bezier_int_created::BezierIntCreated;

pub struct BezierIntCreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<BezierIntCreated> for BezierIntCreatedPublisher {
    fn consume(&self, event: &BezierIntCreated) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::bezier_int_created::BezierIntCreated::new(
            event.id,
        ));
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for BezierIntCreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
