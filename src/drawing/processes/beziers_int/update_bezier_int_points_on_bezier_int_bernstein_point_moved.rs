use crate::drawing::drawing_storage::DrawingStorage;
use backend::cqrs::beziers_int::bezier_int_bernstein_points::BezierIntBernsteinPoints;
use backend::cqrs::cqrs::CQRS;
use backend_events::bezier_int_bernstein_point_moved::BezierIntBernsteinPointMoved;
use glium::glutin::surface::WindowSurface;
use glium::Display;
use infrastructure::consumer::{AnyConsumer, Consumer};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct UpdateBezierIntPointsOnBezierIntBernsteinPointMoved {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<BezierIntBernsteinPointMoved>
    for UpdateBezierIntPointsOnBezierIntBernsteinPointMoved
{
    fn consume(&self, event: &BezierIntBernsteinPointMoved) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let bezier = drawing_storage
            .beziers_int
            .get_mut(&event.bezier_id)
            .unwrap();
        let points = self.cqrs.get(&BezierIntBernsteinPoints {
            id: event.bezier_id,
        });
        bezier.update_points(&points, &self.display);
    }
}

impl AnyConsumer for UpdateBezierIntPointsOnBezierIntBernsteinPointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
