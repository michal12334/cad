use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::beziers_c0::bezier_c0_points::BezierC0Points;
use backend::cqrs::cqrs::CQRS;
use backend_events::bezier_c0_points_deleted::BezierC0PointsDeleted;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct DeleteBezierC0PointsOnBezierC0PointsDeleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<BezierC0PointsDeleted> for DeleteBezierC0PointsOnBezierC0PointsDeleted {
    fn consume(&self, event: &BezierC0PointsDeleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let points = self.cqrs.get(&BezierC0Points { id: event.id });
        let bezier_c0 = drawing_storage.beziers_c0.get_mut(&event.id).unwrap();
        bezier_c0.update_points(&points, &self.display);
    }
}

impl AnyConsumer for DeleteBezierC0PointsOnBezierC0PointsDeleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
