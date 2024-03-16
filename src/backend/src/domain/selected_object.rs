pub struct SelectedObject {
    pub torus_id: u64,
}

impl SelectedObject {
    pub fn new(torus_id: u64) -> Self {
        Self { torus_id }
    }
}
