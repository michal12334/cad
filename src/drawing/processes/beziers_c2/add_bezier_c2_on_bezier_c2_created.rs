use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use backend::cqrs::beziers_c2::bezier_c2_bernstein_points::BezierC2BernsteinPoints;
use backend::cqrs::cqrs::CQRS;
use backend_events::bezier_c2_created::BezierC2Created;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::domain::bezier_c2::BezierC2;
use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddBezierC2OnBezierC2Created {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>
}

impl Consumer<BezierC2Created> for AddBezierC2OnBezierC2Created {
    fn consume(&self, event: &BezierC2Created) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let points = self.cqrs.get(&BezierC2BernsteinPoints { id: event.id });
        drawing_storage.beziers_c2.insert(event.id, BezierC2::new(event.id, &points, &self.display));
    }
}

impl AnyConsumer for AddBezierC2OnBezierC2Created {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
