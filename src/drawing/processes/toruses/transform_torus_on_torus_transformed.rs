use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::toruses::torus_transformed::TorusTransformed;

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct TransformTorusOnTorusTransformed {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<TorusTransformed> for TransformTorusOnTorusTransformed {
    fn consume(&self, event: &TorusTransformed) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage
            .toruses
            .get_mut(&event.id)
            .unwrap()
            .transform(event.position, event.rotation, event.scale);
    }
}

impl AnyConsumer for TransformTorusOnTorusTransformed {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
