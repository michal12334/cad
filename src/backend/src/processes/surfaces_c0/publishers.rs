use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::surfaces_c0::surface_c0_created::SurfaceC0Created;
use crate::domain::events::surfaces_c0::surface_c0_deleted::SurfaceC0Deleted;
use crate::domain::events::surfaces_c0::surface_c0_points_selected::SurfaceC0PointsSelected;
use crate::domain::events::surfaces_c0::surface_c0_updated::SurfaceC0Updated;

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

pub struct SurfaceC0PointsSelectedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<SurfaceC0PointsSelected> for SurfaceC0PointsSelectedPublisher {
    fn consume(&self, message: &SurfaceC0PointsSelected) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::surfaces_c0::surface_c0_points_selected::SurfaceC0PointsSelected::new(message.surface_id),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for SurfaceC0PointsSelectedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct SurfaceC0UpdatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<SurfaceC0Updated> for SurfaceC0UpdatedPublisher {
    fn consume(&self, message: &SurfaceC0Updated) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::surfaces_c0::surface_c0_updated::SurfaceC0Updated::new(message.id, message.draw_polygon, message.tess_level),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for SurfaceC0UpdatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct SurfaceC0DeletedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<SurfaceC0Deleted> for SurfaceC0DeletedPublisher {
    fn consume(&self, message: &SurfaceC0Deleted) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::surfaces_c0::surface_c0_deleted::SurfaceC0Deleted::new(message.id),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for SurfaceC0DeletedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
