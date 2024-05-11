use std::cell::RefCell;
use std::rc::Rc;

use infrastructure::event_bus::EventBus;

use crate::data_access::storage::Storage;
use crate::services::services::Services;

pub struct Backend {
    pub services: Services,
    pub storage: Storage,
}

impl Backend {
    pub fn new(event_bus: Rc<RefCell<EventBus>>) -> Self {
        Backend {
            services: Services::new(event_bus),
            storage: Storage::new(),
        }
    }
}
