pub struct BezierC0PointsDeleted {
    pub id: u64,
    pub deleted_points: Vec<u64>,
}

impl BezierC0PointsDeleted {
    pub fn new(id: u64, deleted_points: Vec<u64>) -> Self {
        Self { id, deleted_points }
    }
}
