use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::surfaces_c0::surface_c0_deleted::SurfaceC0Deleted;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct DeleteSurfaceC0OnSurfaceC0Deleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<SurfaceC0Deleted> for DeleteSurfaceC0OnSurfaceC0Deleted {
    fn consume(&self, event: &SurfaceC0Deleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.surfaces_c0.remove(&event.id);
    }
}

impl AnyConsumer for DeleteSurfaceC0OnSurfaceC0Deleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
