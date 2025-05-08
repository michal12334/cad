use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend::cqrs::cqrs::CQRS;
use backend::cqrs::points::point_details::PointDetails;
use backend_events::points::point_created::PointCreated;
use backend_events::points::point_moved::PointMoved;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::object::Object;
use crate::ui::Ui;

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

pub struct SyncPointCreationWithBackend {
    pub ui: Rc<RefCell<Ui>>,
    pub cqrs: CQRS,
}

impl Consumer<PointCreated> for SyncPointCreationWithBackend {
    fn consume(&self, event: &PointCreated) {
        let mut ui = self.ui.borrow_mut();
        ui.objects
            .push(Object::Point(self.cqrs.get(&PointDetails { id: event.id })));
    }
}

impl AnyConsumer for SyncPointCreationWithBackend {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
