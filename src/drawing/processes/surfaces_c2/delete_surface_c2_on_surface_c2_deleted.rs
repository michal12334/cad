use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend_events::surfaces_c2::surface_c2_deleted::SurfaceC2Deleted;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::drawing_storage::DrawingStorage;

pub struct DeleteSurfaceC2OnSurfaceC2Deleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<SurfaceC2Deleted> for DeleteSurfaceC2OnSurfaceC2Deleted {
    fn consume(&self, event: &SurfaceC2Deleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.surfaces_c2.remove(&event.id);
    }
}

impl AnyConsumer for DeleteSurfaceC2OnSurfaceC2Deleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
