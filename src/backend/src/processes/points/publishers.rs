use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::backend::Backend;
use crate::domain::events::points::point_created::PointCreated;
use crate::domain::events::points::point_moved::PointMoved;

pub struct PointMovedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointMoved> for PointMovedPublisher {
    fn consume(&self, event: &PointMoved) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::points::point_moved::PointMoved::new(
            event.id,
            event.position,
        ));
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for PointMovedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct PointCreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointCreated> for PointCreatedPublisher {
    fn consume(&self, event: &PointCreated) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::points::point_created::PointCreated::new(
            event.id,
            event.name.clone(),
        ));
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for PointCreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
