use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::beziers_int::bezier_int_bernstein_points::BezierIntBernsteinPoints;
use backend::cqrs::cqrs::CQRS;
use backend_events::beziers_int::bezier_int_points_deleted::BezierIntPointsDeleted;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct DeleteBezierIntPointsOnBezierIntPointsDeleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<BezierIntPointsDeleted> for DeleteBezierIntPointsOnBezierIntPointsDeleted {
    fn consume(&self, event: &BezierIntPointsDeleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let bezier = drawing_storage.beziers_int.get_mut(&event.id).unwrap();
        let points = self.cqrs.get(&BezierIntBernsteinPoints { id: event.id });
        bezier.update_points(&points, &self.display);
    }
}

impl AnyConsumer for DeleteBezierIntPointsOnBezierIntPointsDeleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
