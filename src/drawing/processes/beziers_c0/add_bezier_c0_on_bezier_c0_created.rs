use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::beziers_c0::bezier_c0_points::BezierC0Points;
use backend::cqrs::cqrs::CQRS;
use backend_events::beziers_c0::bezier_c0_created::BezierC0Created;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::domain::bezier_c0::BezierC0;
use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddBezierC0OnBezierC0Created {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<BezierC0Created> for AddBezierC0OnBezierC0Created {
    fn consume(&self, event: &BezierC0Created) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let points = self.cqrs.get(&BezierC0Points { id: event.id });
        drawing_storage
            .beziers_c0
            .insert(event.id, BezierC0::new(event.id, &points, &self.display));
    }
}

impl AnyConsumer for AddBezierC0OnBezierC0Created {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
