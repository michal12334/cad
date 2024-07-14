pub struct PointMoved {
    pub id: u64,
    pub position: (f64, f64, f64),
}

impl PointMoved {
    pub fn new(id: u64, position: (f64, f64, f64)) -> Self {
        Self { id, position }
    }
}
