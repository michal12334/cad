use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::surfaces_c2::surface_c2_created::SurfaceC2Created;
use crate::domain::events::surfaces_c2::surface_c2_deleted::SurfaceC2Deleted;
use crate::domain::events::surfaces_c2::surface_c2_points_selected::SurfaceC2PointsSelected;
use crate::domain::events::surfaces_c2::surface_c2_updated::SurfaceC2Updated;

pub struct SurfaceC2CreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<SurfaceC2Created> for SurfaceC2CreatedPublisher {
    fn consume(&self, message: &SurfaceC2Created) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::surfaces_c2::surface_c2_created::SurfaceC2Created::new(message.id, message.size),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for SurfaceC2CreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct SurfaceC2PointsSelectedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<SurfaceC2PointsSelected> for SurfaceC2PointsSelectedPublisher {
    fn consume(&self, message: &SurfaceC2PointsSelected) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::surfaces_c2::surface_c2_points_selected::SurfaceC2PointsSelected::new(message.surface_id),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for SurfaceC2PointsSelectedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct SurfaceC2UpdatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<SurfaceC2Updated> for SurfaceC2UpdatedPublisher {
    fn consume(&self, message: &SurfaceC2Updated) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::surfaces_c2::surface_c2_updated::SurfaceC2Updated::new(message.id, message.draw_polygon, message.tess_level),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for SurfaceC2UpdatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct SurfaceC2DeletedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<SurfaceC2Deleted> for SurfaceC2DeletedPublisher {
    fn consume(&self, message: &SurfaceC2Deleted) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::surfaces_c2::surface_c2_deleted::SurfaceC2Deleted::new(message.id),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for SurfaceC2DeletedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
