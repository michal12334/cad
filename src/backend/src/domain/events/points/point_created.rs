pub struct PointCreated {
    pub id: u64,
    pub name: String,
}

impl PointCreated {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}
