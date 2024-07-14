use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::beziers_c0::bezier_c0_draw_polygon_set::BezierC0DrawPolygonSet;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct SetDrawPolygonOnBezierC0DrawPolygonSet {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<BezierC0DrawPolygonSet> for SetDrawPolygonOnBezierC0DrawPolygonSet {
    fn consume(&self, event: &BezierC0DrawPolygonSet) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let bezier_c0 = drawing_storage
            .beziers_c0
            .get_mut(&event.bezier_id)
            .unwrap();
        bezier_c0.draw_polygon = event.draw_polygon;
    }
}

impl AnyConsumer for SetDrawPolygonOnBezierC0DrawPolygonSet {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
