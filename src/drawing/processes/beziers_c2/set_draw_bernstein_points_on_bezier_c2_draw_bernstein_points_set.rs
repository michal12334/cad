use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend_events::bezier_c2_draw_bernstein_points_set::BezierC2DrawBernsteinPointsSet;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::drawing_storage::DrawingStorage;

pub struct SetDrawBernsteinPointsOnBezierC2DrawBernsteinPointsSet {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<BezierC2DrawBernsteinPointsSet> for SetDrawBernsteinPointsOnBezierC2DrawBernsteinPointsSet {
    fn consume(&self, event: &BezierC2DrawBernsteinPointsSet) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let bezier_c2 = drawing_storage.beziers_c2.get_mut(&event.bezier_id).unwrap();
        bezier_c2.draw_bernstein_points = event.draw_bernstein_points;
    }
}

impl AnyConsumer for SetDrawBernsteinPointsOnBezierC2DrawBernsteinPointsSet {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
