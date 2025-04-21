use std::{any::Any, cell::RefCell, rc::Rc};

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::{
    backend::Backend, domain::events::intersections::intersection_created::IntersectionCreated,
};

pub struct IntersectionCreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<IntersectionCreated> for IntersectionCreatedPublisher {
    fn consume(&self, event: &IntersectionCreated) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::intersections::intersection_created::IntersectionCreated::new(
                event.id,
                event.name.clone(),
                event.uv_texture.clone(),
                event.st_texture.clone(),
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for IntersectionCreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
