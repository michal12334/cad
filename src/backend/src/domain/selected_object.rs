pub struct SelectedObject {
    pub torus_id: Option<u64>,
    pub point_id: Option<u64>,
}

impl SelectedObject {
    pub fn new_torus(torus_id: u64) -> Self {
        Self {
            torus_id: Some(torus_id),
            point_id: None,
        }
    }

    pub fn new_point(point_id: u64) -> Self {
        Self {
            torus_id: None,
            point_id: Some(point_id),
        }
    }
}
