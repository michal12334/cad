use std::{any::Any, cell::RefCell, rc::Rc};

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::{
    backend::Backend,
    domain::events::gregories::{gregory_created::GregoryCreated, gregory_renamed::GregoryRenamed},
};

pub struct GregoryCreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<GregoryCreated> for GregoryCreatedPublisher {
    fn consume(&self, event: &GregoryCreated) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::gregories::gregory_created::GregoryCreated::new(
                event.gregory_id,
                event.name.clone(),
                event.tess_level,
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for GregoryCreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct GregoryRenamedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<GregoryRenamed> for GregoryRenamedPublisher {
    fn consume(&self, event: &GregoryRenamed) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::gregories::gregory_renamed::GregoryRenamed::new(
                event.gregory_id,
                event.name.clone(),
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for GregoryRenamedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
