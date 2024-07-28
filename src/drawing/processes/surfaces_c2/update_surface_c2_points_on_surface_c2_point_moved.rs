use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::surfaces_c2::surface_c2_points::SurfaceC2Points;
use backend_events::surfaces_c2::surface_c2_point_moved::SurfaceC2PointMoved;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateSurfaceC2PointsOnSurfaceC2PointMoved {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<SurfaceC2PointMoved> for UpdateSurfaceC2PointsOnSurfaceC2PointMoved {
    fn consume(&self, event: &SurfaceC2PointMoved) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let surface = drawing_storage
            .surfaces_c2
            .get_mut(&event.id)
            .unwrap();
        let points = self.cqrs.get(&SurfaceC2Points {
            id: event.id,
        });
        surface.update_points(&points, &self.display);
    }
}

impl AnyConsumer for UpdateSurfaceC2PointsOnSurfaceC2PointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
