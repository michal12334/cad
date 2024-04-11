use crate::data_access::storage::Storage;
use crate::services::services::Services;

pub struct Backend {
    pub services: Services,
    pub storage: Storage,
}

impl Backend {
    pub fn new() -> Self {
        Backend {
            services: Services::new(),
            storage: Storage::new(),
        }
    }
}
