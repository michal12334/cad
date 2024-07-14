use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::backend::Backend;
use crate::domain::events::beziers_c2::bezier_c2_point_moved::BezierC2PointMoved;
use crate::domain::events::points::point_moved::PointMoved;

pub struct MoveBezierC2PointsOnPointMoved {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointMoved> for MoveBezierC2PointsOnPointMoved {
    fn consume(&self, event: &PointMoved) {
        let backend = self.backend.as_ptr();
        let storage = unsafe { &mut (*backend).storage };
        let publisher = unsafe { &(*backend).services.event_publisher };
        storage.beziers_c2.values_mut().for_each(|bezier_c2| {
            if bezier_c2
                .b_spline_points
                .iter()
                .any(|point| point.id == event.id)
            {
                let storage = unsafe { &mut (*backend).storage };
                let points: Vec<_> = bezier_c2
                    .b_spline_points
                    .iter()
                    .map(|p| storage.points.get(&p.id).unwrap().clone())
                    .collect();
                bezier_c2.update_points(points);
                publisher.publish(Rc::new(BezierC2PointMoved::new(bezier_c2.id)));
            }
        });
    }
}

impl AnyConsumer for MoveBezierC2PointsOnPointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
