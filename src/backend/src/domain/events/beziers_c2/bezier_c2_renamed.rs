use crate::domain::bezier_c2::BezierC2;

pub struct BezierC2Renamed {
    pub id: u64,
    pub name: String,
}

impl BezierC2Renamed {
    pub fn new(bezier: &BezierC2) -> Self {
        Self {
            id: bezier.id,
            name: bezier.name.clone(),
        }
    }
}
