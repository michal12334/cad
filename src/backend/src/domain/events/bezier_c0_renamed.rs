use crate::domain::bezier_c0::BezierC0;

pub struct BezierC0Renamed {
    pub id: u64,
    pub name: String,
}

impl BezierC0Renamed {
    pub fn new(bezier: &BezierC0) -> Self {
        Self { id: bezier.id, name: bezier.name.clone() }
    }
}
