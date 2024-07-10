use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::cqrs::CQRS;
use backend::cqrs::points::point_details::PointDetails;
use backend_events::point_added_to_bezier_c0::PointAddedToBezierC0;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddPointToBezierC0OnPointAddedToBezierC0 {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<PointAddedToBezierC0> for AddPointToBezierC0OnPointAddedToBezierC0 {
    fn consume(&self, event: &PointAddedToBezierC0) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let point = self.cqrs.get(&PointDetails { id: event.point_id });
        let bezier_c0 = drawing_storage
            .beziers_c0
            .get_mut(&event.bezier_id)
            .unwrap();
        bezier_c0.add_point(point, &self.display);
    }
}

impl AnyConsumer for AddPointToBezierC0OnPointAddedToBezierC0 {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
