pub struct BezierC2SelectedBernsteinPointSet {
    pub bezier_id: u64,
    pub selected_bernstein_point: Option<usize>,
}

impl BezierC2SelectedBernsteinPointSet {
    pub fn new(bezier_id: u64, selected_bernstein_point: Option<usize>) -> Self {
        Self { bezier_id, selected_bernstein_point, }
    }
}
