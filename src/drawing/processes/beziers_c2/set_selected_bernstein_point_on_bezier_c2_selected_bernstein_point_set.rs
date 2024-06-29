use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend_events::bezier_c2_selected_bernstein_point_set::BezierC2SelectedBernsteinPointSet;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::drawing_storage::DrawingStorage;

pub struct SetSelectedBernsteinPointOnBezierC2SelectedBernsteinPointSet {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<BezierC2SelectedBernsteinPointSet> for SetSelectedBernsteinPointOnBezierC2SelectedBernsteinPointSet {
    fn consume(&self, event: &BezierC2SelectedBernsteinPointSet) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let bezier_c2 = drawing_storage.beziers_c2.get_mut(&event.bezier_id).unwrap();
        bezier_c2.selected_bernstein_point = event.selected_bernstein_point;
    }
}

impl AnyConsumer for SetSelectedBernsteinPointOnBezierC2SelectedBernsteinPointSet {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
