use std::{any::Any, cell::RefCell, rc::Rc};

use backend_events::gregories::gregory_deleted::GregoryDeleted;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct DeleteGregoryOnGregoryDeleted {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<GregoryDeleted> for DeleteGregoryOnGregoryDeleted {
    fn consume(&self, message: &GregoryDeleted) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.gregories.remove(&message.gregory_id);
    }
}

impl AnyConsumer for DeleteGregoryOnGregoryDeleted {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
