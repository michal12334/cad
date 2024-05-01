use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::bezier_c0_points_deleted::BezierC0PointsDeleted;
use crate::domain::events::bezier_c0_renamed::BezierC0Renamed;
use crate::domain::events::point_added_to_bezier_c0::PointAddedToBezierC0;

pub struct BezierC0RenamedPublisher {
    pub backend: Rc<RefCell<Backend>>
}

impl Consumer<BezierC0Renamed> for BezierC0RenamedPublisher {
    fn consume(&self, event: &BezierC0Renamed) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::bezier_c0_renamed::BezierC0Renamed::new(event.id, event.name.clone()));
        backend.services.event_publisher.publish(event);
    }
}

pub struct BezierC0PointsDeletedPublisher {
    pub backend: Rc<RefCell<Backend>>
}

impl Consumer<BezierC0PointsDeleted> for BezierC0PointsDeletedPublisher {
    fn consume(&self, event: &BezierC0PointsDeleted) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::bezier_c0_points_deleted::BezierC0PointsDeleted::new(event.id, event.deleted_points.clone()));
        backend.services.event_publisher.publish(event);
    }
}

pub struct PointAddedToBezierC0Publisher {
    pub backend: Rc<RefCell<Backend>>
}

impl Consumer<PointAddedToBezierC0> for PointAddedToBezierC0Publisher {
    fn consume(&self, event: &PointAddedToBezierC0) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::point_added_to_bezier_c0::PointAddedToBezierC0::new(event.point_id, event.bezier_id, event.point_name.clone()));
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for BezierC0RenamedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for BezierC0PointsDeletedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for PointAddedToBezierC0Publisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
