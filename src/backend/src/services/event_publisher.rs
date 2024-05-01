use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::event_bus::EventBus;

pub struct EventPublisher {
    pub bus: Rc<RefCell<EventBus>>,
}

impl EventPublisher {
    pub fn new(bus: Rc<RefCell<EventBus>>) -> Self {
        Self {
            bus,
        }
    }

    pub fn publish<T : Any>(&self, message: Rc<T>) {
        self.bus.borrow().publish(message);
    }
}
