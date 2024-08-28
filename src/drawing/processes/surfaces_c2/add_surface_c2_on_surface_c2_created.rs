use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::cqrs::CQRS;
use backend::cqrs::surfaces_c2::surface_c2_points::SurfaceC2Points;
use backend_events::surfaces_c2::surface_c2_created::SurfaceC2Created;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::domain::surface_c2::SurfaceC2;
use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddSurfaceC2OnSurfaceC2Created {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<SurfaceC2Created> for AddSurfaceC2OnSurfaceC2Created {
    fn consume(&self, event: &SurfaceC2Created) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let points = self.cqrs.get(&SurfaceC2Points { id: event.id });
        drawing_storage.surfaces_c2.insert(
            event.id,
            SurfaceC2::new(event.id, &points, event.size, &self.display),
        );
    }
}

impl AnyConsumer for AddSurfaceC2OnSurfaceC2Created {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
