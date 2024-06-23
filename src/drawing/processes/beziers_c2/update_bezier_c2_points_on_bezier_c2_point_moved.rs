use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use backend::cqrs::beziers_c2::bezier_c2_b_spline_points::BezierC2BSplinePoints;
use backend::cqrs::beziers_c2::bezier_c2_bernstein_points::BezierC2BernsteinPoints;
use backend::cqrs::cqrs::CQRS;
use backend_events::bezier_c2_point_moved::BezierC2PointMoved;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateBezierC2PointsOnBezierC2PointMoved {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>
}

impl Consumer<BezierC2PointMoved> for UpdateBezierC2PointsOnBezierC2PointMoved {
    fn consume(&self, event: &BezierC2PointMoved) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let bernstein_points = self.cqrs.get(&BezierC2BernsteinPoints { id: event.bezier_id });
        let b_spline_points = self.cqrs.get(&BezierC2BSplinePoints { id: event.bezier_id });
        let bezier_c2 = drawing_storage.beziers_c2.get_mut(&event.bezier_id).unwrap();
        bezier_c2.update_points(&bernstein_points, &b_spline_points, &self.display);
    }
}

impl AnyConsumer for UpdateBezierC2PointsOnBezierC2PointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

