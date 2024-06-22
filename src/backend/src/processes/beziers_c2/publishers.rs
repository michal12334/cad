use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::bezier_c2_created::BezierC2Created;
use crate::domain::events::bezier_c2_draw_b_spline_polygon_set::BezierC2DrawBSplinePolygonSet;
use crate::domain::events::bezier_c2_draw_bernstein_points_set::BezierC2DrawBernsteinPointsSet;
use crate::domain::events::bezier_c2_draw_bernstein_polygon_set::BezierC2DrawBernsteinPolygonSet;
use crate::domain::events::point_added_to_bezier_c2::PointAddedToBezierC2;

pub struct BezierC2CreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<BezierC2Created> for BezierC2CreatedPublisher {
    fn consume(&self, event: &BezierC2Created) {
        let backend = self.backend.borrow();
        let event = Rc::new(backend_events::bezier_c2_created::BezierC2Created::new(
            event.id,
        ));
        backend.services.event_publisher.publish(event);
    }
}

pub struct PointAddedToBezierC2Publisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointAddedToBezierC2> for PointAddedToBezierC2Publisher {
    fn consume(&self, event: &PointAddedToBezierC2) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::point_added_to_bezier_c2::PointAddedToBezierC2::new(
                event.point_id,
                event.bezier_id,
                event.point_name.clone(),
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

pub struct BezierC2DrawBernsteinPolygonSetPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<BezierC2DrawBernsteinPolygonSet> for BezierC2DrawBernsteinPolygonSetPublisher {
    fn consume(&self, event: &BezierC2DrawBernsteinPolygonSet) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::bezier_c2_draw_bernstein_polygon_set::BezierC2DrawBernsteinPolygonSet::new(
                event.bezier_id,
                event.draw_bernstein_polygon,
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

pub struct BezierC2DrawBernsteinPointsSetPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<BezierC2DrawBernsteinPointsSet> for BezierC2DrawBernsteinPointsSetPublisher {
    fn consume(&self, event: &BezierC2DrawBernsteinPointsSet) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::bezier_c2_draw_bernstein_points_set::BezierC2DrawBernsteinPointsSet::new(
                event.bezier_id,
                event.draw_bernstein_points,
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

pub struct BezierC2DrawBSplinePolygonSetPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<BezierC2DrawBSplinePolygonSet> for BezierC2DrawBSplinePolygonSetPublisher {
    fn consume(&self, event: &BezierC2DrawBSplinePolygonSet) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::bezier_c2_draw_b_spline_polygon_set::BezierC2DrawBSplinePolygonSet::new(
                event.bezier_id,
                event.draw_b_spline_polygon,
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for BezierC2CreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for PointAddedToBezierC2Publisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for BezierC2DrawBernsteinPolygonSetPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for BezierC2DrawBernsteinPointsSetPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for BezierC2DrawBSplinePolygonSetPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
