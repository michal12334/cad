use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::toruses::torus_created::TorusCreated;
use glium::glutin::surface::WindowSurface;
use glium::Display;

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::domain::torus::Torus;
use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddTorusOnTorusCreated {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<TorusCreated> for AddTorusOnTorusCreated {
    fn consume(&self, event: &TorusCreated) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.toruses.insert(
            event.id,
            Torus::new(
                event.id,
                event.major_radius,
                event.minor_radius,
                event.major_segments,
                event.minor_segments,
                event.position,
                event.rotation,
                event.scale,
                &self.display,
            ),
        );
    }
}

impl AnyConsumer for AddTorusOnTorusCreated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
