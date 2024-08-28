use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::cqrs::CQRS;
use backend::cqrs::surfaces_c0::surface_c0_points::SurfaceC0Points;
use backend_events::surfaces_c0::surface_c0_point_moved::SurfaceC0PointMoved;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateSurfaceC0PointsOnSurfaceC0PointMoved {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<SurfaceC0PointMoved> for UpdateSurfaceC0PointsOnSurfaceC0PointMoved {
    fn consume(&self, event: &SurfaceC0PointMoved) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let surface = drawing_storage.surfaces_c0.get_mut(&event.id).unwrap();
        let points = self.cqrs.get(&SurfaceC0Points { id: event.id });
        surface.update_points(&points, &self.display);
    }
}

impl AnyConsumer for UpdateSurfaceC0PointsOnSurfaceC0PointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
