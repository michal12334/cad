use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::beziers_int::bezier_int_bernstein_points::BezierIntBernsteinPoints;
use backend::cqrs::cqrs::CQRS;
use backend_events::bezier_int_created::BezierIntCreated;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::domain::bezier_int::BezierInt;
use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddBezierIntOnBezierIntCreated {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<BezierIntCreated> for AddBezierIntOnBezierIntCreated {
    fn consume(&self, event: &BezierIntCreated) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let points = self.cqrs.get(&BezierIntBernsteinPoints { id: event.id });
        drawing_storage
            .beziers_int
            .insert(event.id, BezierInt::new(event.id, &points, &self.display));
    }
}

impl AnyConsumer for AddBezierIntOnBezierIntCreated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
