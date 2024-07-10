use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::bezier_int_bernstein_point_moved::BezierIntBernsteinPointMoved;
use crate::domain::events::point_moved::PointMoved;

pub struct UpdateBezierIntPointsOnPointMoved {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointMoved> for UpdateBezierIntPointsOnPointMoved {
    fn consume(&self, event: &PointMoved) {
        let backend = self.backend.as_ptr();
        let storage = unsafe { &mut (*backend).storage };
        let publisher = unsafe { &(*backend).services.event_publisher };
        storage
            .beziers_int
            .values_mut()
            .for_each(|bezier_int| {
                if 
                bezier_int.points
                    .iter()
                    .any(|point| point.id == event.id)
                {
                    let storage = unsafe { &mut (*backend).storage };
                    let points: Vec<_> = bezier_int.points.iter().map(|p| storage.points.get(&p.id).unwrap().clone()).collect();
                    bezier_int.update_points(points);
                    publisher.publish(Rc::new(BezierIntBernsteinPointMoved::new(bezier_int.id)));
                }
            });
    }
}

impl AnyConsumer for UpdateBezierIntPointsOnPointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
