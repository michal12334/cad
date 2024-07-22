use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::surfaces_c0::surface_c0_created::SurfaceC0Created;

pub struct SurfaceC0CreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<SurfaceC0Created> for SurfaceC0CreatedPublisher {
    fn consume(&self, message: &SurfaceC0Created) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::surfaces_c0::surface_c0_created::SurfaceC0Created::new(message.id, message.size),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for SurfaceC0CreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
