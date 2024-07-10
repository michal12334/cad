use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::bezier_int_created::BezierIntCreated;
use crate::domain::events::bezier_int_points_deleted::BezierIntPointsDeleted;
use crate::domain::events::point_added_to_bezier_int::PointAddedToBezierInt;

pub struct BezierIntCreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<BezierIntCreated> for BezierIntCreatedPublisher {
    fn consume(&self, event: &BezierIntCreated) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::bezier_int_created::BezierIntCreated::new(
            event.id,
        ));
        backend.services.event_publisher.publish(event);
    }
}

pub struct PointAddedToBezierIntPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointAddedToBezierInt> for PointAddedToBezierIntPublisher {
    fn consume(&self, event: &PointAddedToBezierInt) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::point_added_to_bezier_int::PointAddedToBezierInt::new(
            event.point_id,
            event.bezier_id,
            event.point_name.clone(),
        ));
        backend.services.event_publisher.publish(event);
    }
}

pub struct BezierIntPointsDeletedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<BezierIntPointsDeleted> for BezierIntPointsDeletedPublisher {
    fn consume(&self, event: &BezierIntPointsDeleted) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::bezier_int_points_deleted::BezierIntPointsDeleted::new(
            event.id,
            event.deleted_points.clone(),
        ));
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for BezierIntCreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for PointAddedToBezierIntPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for BezierIntPointsDeletedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
