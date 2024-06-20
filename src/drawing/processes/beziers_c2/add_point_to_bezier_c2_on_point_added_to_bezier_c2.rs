use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use backend::cqrs::beziers_c2::bezier_c2_bernstein_points::BezierC2BernsteinPoints;
use backend::cqrs::cqrs::CQRS;
use backend_events::point_added_to_bezier_c2::PointAddedToBezierC2;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddPointToBezierC2OnPointAddedToBezierC2 {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>
}

impl Consumer<PointAddedToBezierC2> for AddPointToBezierC2OnPointAddedToBezierC2 {
    fn consume(&self, event: &PointAddedToBezierC2) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let points = self.cqrs.get(&BezierC2BernsteinPoints { id: event.bezier_id });
        let bezier_c2 = drawing_storage.beziers_c2.get_mut(&event.bezier_id).unwrap();
        bezier_c2.update_points(&points, &self.display);
    }
}

impl AnyConsumer for AddPointToBezierC2OnPointAddedToBezierC2 {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
