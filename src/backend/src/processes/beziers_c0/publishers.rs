use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::bezier_c0_renamed::BezierC0Renamed;

pub struct BezierC0RenamedPublisher {
    pub backend: Rc<RefCell<Backend>>
}

impl AnyConsumer for BezierC0RenamedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl Consumer<BezierC0Renamed> for BezierC0RenamedPublisher {
    fn consume(&self, event: &BezierC0Renamed) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::bezier_c0_renamed::BezierC0Renamed::new(event.id, event.name.clone()));
        backend.services.event_publisher.publish(event);
    }
}
