use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::bezier_c0_point_moved::BezierC0PointMoved;
use crate::domain::events::point_moved::PointMoved;

pub struct MoveBezierC0PointsOnPointMoved {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointMoved> for MoveBezierC0PointsOnPointMoved {
    fn consume(&self, event: &PointMoved) {
        let backend = self.backend.as_ptr();
        let storage = unsafe { &mut (*backend).storage };
        let publisher = unsafe { &(*backend).services.event_publisher };
        storage
            .beziers_c0
            .values()
            .for_each(|bezier_c0| {
                bezier_c0.points
                    .iter()
                    .filter(|point| point.id == event.id)
                    .for_each(|point| {
                        publisher.publish(Rc::new(BezierC0PointMoved::new(bezier_c0.id, point.id, event.position)));
                    });
            });
    }
}

impl AnyConsumer for MoveBezierC0PointsOnPointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
