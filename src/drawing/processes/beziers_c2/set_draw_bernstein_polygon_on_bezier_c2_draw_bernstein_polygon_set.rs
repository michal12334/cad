use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend_events::bezier_c2_draw_bernstein_polygon_set::BezierC2DrawBernsteinPolygonSet;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::drawing_storage::DrawingStorage;

pub struct SetDrawBernsteinPolygonOnBezierC2DrawBernsteinPolygonSet {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<BezierC2DrawBernsteinPolygonSet> for SetDrawBernsteinPolygonOnBezierC2DrawBernsteinPolygonSet {
    fn consume(&self, event: &BezierC2DrawBernsteinPolygonSet) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let bezier_c2 = drawing_storage.beziers_c2.get_mut(&event.bezier_id).unwrap();
        bezier_c2.draw_bernstein_polygon = event.draw_bernstein_polygon;
    }
}

impl AnyConsumer for SetDrawBernsteinPolygonOnBezierC2DrawBernsteinPolygonSet {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
