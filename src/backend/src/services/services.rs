use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::event_bus::EventBus;
use crate::services::event_publisher::EventPublisher;
use crate::services::id_generator::IdGenerator;

pub struct Services {
    pub id_generator: IdGenerator,
    pub event_publisher: EventPublisher,
}

impl Services {
    pub fn new(event_bus: Rc<RefCell<EventBus>>) -> Self {
        Self {
            id_generator: IdGenerator::new(),
            event_publisher: EventPublisher::new(event_bus),
        }
    }
}
