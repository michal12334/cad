pub struct SelectedObject {
    pub torus_id: Option<u64>,
    pub point_id: Option<u64>,
}

impl SelectedObject {
    pub fn new(torus_id: u64) -> Self {
        Self { 
            torus_id: Some(torus_id),
            point_id: Some(torus_id),
        }
    }
}
