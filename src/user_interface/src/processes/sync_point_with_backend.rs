use crate::object::Object;
use crate::ui::Ui;
use backend_events::point_moved::PointMoved;
use infrastructure::consumer::{AnyConsumer, Consumer};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SyncPointPositionWithBackend {
    pub ui: Rc<RefCell<Ui>>,
}

impl Consumer<PointMoved> for SyncPointPositionWithBackend {
    fn consume(&self, event: &PointMoved) {
        let mut ui = self.ui.borrow_mut();
        ui.objects
            .iter_mut()
            .filter(|object| object.get_id() == event.id)
            .for_each(|object| match object {
                Object::Point(point) => {
                    point.transformer.position = event.position;
                }
                _ => {}
            });
    }
}

impl AnyConsumer for SyncPointPositionWithBackend {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
