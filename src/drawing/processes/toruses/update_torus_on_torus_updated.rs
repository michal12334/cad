use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::toruses::torus_updated::TorusUpdated;
use glium::glutin::surface::WindowSurface;
use glium::Display;

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateTorusOnTorusUpdated {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<TorusUpdated> for UpdateTorusOnTorusUpdated {
    fn consume(&self, event: &TorusUpdated) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.toruses.get_mut(&event.id).unwrap().update(
            event.major_radius,
            event.minor_radius,
            event.major_segments,
            event.minor_segments,
            &self.display,
        );
    }
}

impl AnyConsumer for UpdateTorusOnTorusUpdated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
