use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::beziers_c2::bezier_c2_draw_b_spline_polygon_set::BezierC2DrawBSplinePolygonSet;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct SetDrawBSplinePolygonOnBezierC2DrawBSplinePolygonSet {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<BezierC2DrawBSplinePolygonSet>
    for SetDrawBSplinePolygonOnBezierC2DrawBSplinePolygonSet
{
    fn consume(&self, event: &BezierC2DrawBSplinePolygonSet) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let bezier_c2 = drawing_storage
            .beziers_c2
            .get_mut(&event.bezier_id)
            .unwrap();
        bezier_c2.draw_b_spline_polygon = event.draw_b_spline_polygon;
    }
}

impl AnyConsumer for SetDrawBSplinePolygonOnBezierC2DrawBSplinePolygonSet {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
