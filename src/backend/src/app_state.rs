use crate::data_access::storage::Storage;
use crate::services::services::Services;

pub struct AppState {
    pub services: Services,
    pub storage: Storage,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            services: Services::new(),
            storage: Storage::new(),
        }
    }
}
