use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend::cqrs::beziers_c2::bezier_c2_bernstein_points::BezierC2BernsteinPoints;
use backend::cqrs::cqrs::CQRS;
use backend_events::point_added_to_bezier_c2::PointAddedToBezierC2;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::domain::bezier_c2::BezierC2BSplinePoint;
use crate::object::Object;
use crate::ui::Ui;

pub struct SyncBezierC2AddedPointsWithBackend {
    pub ui: Rc<RefCell<Ui>>,
    pub cqrs: CQRS,
}

impl Consumer<PointAddedToBezierC2> for SyncBezierC2AddedPointsWithBackend {
    fn consume(&self, event: &PointAddedToBezierC2) {
        let mut ui = self.ui.borrow_mut();
        ui.objects
            .iter_mut()
            .filter(|object| object.get_id() == event.bezier_id)
            .for_each(|object| match object {
                Object::BezierC2(bezier) => {
                    let point = BezierC2BSplinePoint::new(
                        event.point_id,
                        event.point_name.clone(),
                    );
                    bezier.b_spline_points.push(point);
                    bezier.set_bernstein_points(&self.cqrs.get(&BezierC2BernsteinPoints { id: event.bezier_id }));
                }
                _ => {}
            });
    }
}

impl AnyConsumer for SyncBezierC2AddedPointsWithBackend {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
