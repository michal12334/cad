pub struct PointAddedToBezierInt {
    pub point_id: u64,
    pub bezier_id: u64,
    pub point_name: String,
}

impl PointAddedToBezierInt {
    pub fn new(point_id: u64, bezier_id: u64, point_name: String) -> Self {
        Self {
            point_id,
            bezier_id,
            point_name,
        }
    }
}
