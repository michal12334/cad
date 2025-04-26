use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::intersections::intersection_created::IntersectionCreated;
use glium::glutin::surface::WindowSurface;
use glium::Display;

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::domain::intersection::Intersection;
use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddIntersectionOnIntersectionCreated {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<IntersectionCreated> for AddIntersectionOnIntersectionCreated {
    fn consume(&self, event: &IntersectionCreated) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.intersections.insert(
            event.id,
            Intersection::new(event.id, &event.points, event.wrap, &self.display),
        );
    }
}

impl AnyConsumer for AddIntersectionOnIntersectionCreated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
