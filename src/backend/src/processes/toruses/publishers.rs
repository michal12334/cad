use std::{any::Any, cell::RefCell, rc::Rc};

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::{
    backend::Backend,
    domain::events::toruses::{
        torus_created::TorusCreated, torus_deleted::TorusDeleted,
        torus_transformed::TorusTransformed, torus_updated::TorusUpdated,
    },
};

pub struct TorusCreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<TorusCreated> for TorusCreatedPublisher {
    fn consume(&self, message: &TorusCreated) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::toruses::torus_created::TorusCreated::new(
            message.id,
            message.major_radius,
            message.minor_radius,
            message.major_segments,
            message.minor_segments,
            message.position,
            message.rotation,
            message.scale,
        ));
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for TorusCreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct TorusUpdatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<TorusUpdated> for TorusUpdatedPublisher {
    fn consume(&self, message: &TorusUpdated) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::toruses::torus_updated::TorusUpdated::new(
            message.id,
            message.major_radius,
            message.minor_radius,
            message.major_segments,
            message.minor_segments,
        ));
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for TorusUpdatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct TorusTransformedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<TorusTransformed> for TorusTransformedPublisher {
    fn consume(&self, message: &TorusTransformed) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::toruses::torus_transformed::TorusTransformed::new(
                message.id,
                message.position,
                message.rotation,
                message.scale,
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for TorusTransformedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct TorusDeletedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<TorusDeleted> for TorusDeletedPublisher {
    fn consume(&self, message: &TorusDeleted) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::toruses::torus_deleted::TorusDeleted::new(
            message.id,
        ));
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for TorusDeletedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
