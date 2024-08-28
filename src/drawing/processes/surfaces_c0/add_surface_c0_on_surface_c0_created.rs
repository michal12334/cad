use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::cqrs::CQRS;
use backend::cqrs::surfaces_c0::surface_c0_points::SurfaceC0Points;
use backend_events::surfaces_c0::surface_c0_created::SurfaceC0Created;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::domain::surface_c0::SurfaceC0;
use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddSurfaceC0OnSurfaceC0Created {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<SurfaceC0Created> for AddSurfaceC0OnSurfaceC0Created {
    fn consume(&self, event: &SurfaceC0Created) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let points = self.cqrs.get(&SurfaceC0Points { id: event.id });
        drawing_storage.surfaces_c0.insert(
            event.id,
            SurfaceC0::new(event.id, &points, event.size, &self.display),
        );
    }
}

impl AnyConsumer for AddSurfaceC0OnSurfaceC0Created {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
