use std::{any::Any, cell::RefCell, rc::Rc};

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::{
    backend::Backend,
    domain::events::gregories::{
        gregory_created::GregoryCreated, gregory_deleted::GregoryDeleted,
        gregory_mesh_recalculated::GregoryMeshRecalculated, gregory_renamed::GregoryRenamed,
        gregory_settings_updated::GregorySettingsUpdated,
    },
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
                event.draw_vectors,
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

pub struct GregoryMeshRecalculatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<GregoryMeshRecalculated> for GregoryMeshRecalculatedPublisher {
    fn consume(&self, event: &GregoryMeshRecalculated) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::gregories::gregory_mesh_recalculated::GregoryMeshRecalculated::new(
                event.gregory_id,
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for GregoryMeshRecalculatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct GregorySettingsUpdatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<GregorySettingsUpdated> for GregorySettingsUpdatedPublisher {
    fn consume(&self, event: &GregorySettingsUpdated) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::gregories::gregory_settings_updated::GregorySettingsUpdated::new(
                event.gregory_id,
                event.tess_level,
                event.draw_vectors,
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for GregorySettingsUpdatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct GregoryDeletedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<GregoryDeleted> for GregoryDeletedPublisher {
    fn consume(&self, event: &GregoryDeleted) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::gregories::gregory_deleted::GregoryDeleted::new(event.gregory_id),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for GregoryDeletedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
