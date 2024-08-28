use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::surfaces_c0::surface_c0_updated::SurfaceC0Updated;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateSurfaceC0OnSurfaceC0Updated {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<SurfaceC0Updated> for UpdateSurfaceC0OnSurfaceC0Updated {
    fn consume(&self, event: &SurfaceC0Updated) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let surface_c0 = drawing_storage.surfaces_c0.get_mut(&event.id).unwrap();
        surface_c0.set_draw_polygon(event.draw_polygon);
        surface_c0.set_tess_level(event.tess_level);
    }
}

impl AnyConsumer for UpdateSurfaceC0OnSurfaceC0Updated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
