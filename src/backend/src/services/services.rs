use crate::services::id_generator::IdGenerator;

pub struct Services {
    pub id_generator: IdGenerator,
}

impl Services {
    pub fn new() -> Self {
        Self {
            id_generator: IdGenerator::new(),
        }
    }
}
