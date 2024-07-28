use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend_events::surfaces_c2::surface_c2_updated::SurfaceC2Updated;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateSurfaceC2OnSurfaceC2Updated {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<SurfaceC2Updated> for UpdateSurfaceC2OnSurfaceC2Updated {
    fn consume(&self, event: &SurfaceC2Updated) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let surface_c2 = drawing_storage.surfaces_c2.get_mut(&event.id).unwrap();
        surface_c2.set_draw_polygon(event.draw_polygon);
        surface_c2.set_tess_level(event.tess_level);
    }
}

impl AnyConsumer for UpdateSurfaceC2OnSurfaceC2Updated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
