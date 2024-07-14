use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::beziers_c0::bezier_c0_points::BezierC0Points;
use backend::cqrs::cqrs::CQRS;
use backend_events::beziers_c0::bezier_c0_point_moved::BezierC0PointMoved;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateBezierC0PointsOnBezierC0PointMoved {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<BezierC0PointMoved> for UpdateBezierC0PointsOnBezierC0PointMoved {
    fn consume(&self, event: &BezierC0PointMoved) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let bezier_c0 = drawing_storage
            .beziers_c0
            .get_mut(&event.bezier_id)
            .unwrap();
        let points = self.cqrs.get(&BezierC0Points {
            id: event.bezier_id,
        });
        bezier_c0.update_points(&points, &self.display);
    }
}

impl AnyConsumer for UpdateBezierC0PointsOnBezierC0PointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
