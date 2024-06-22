use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::bezier_c2_draw_b_spline_polygon_set::BezierC2DrawBSplinePolygonSet;

pub struct SetBezierC2DrawBSplinePolygon {
    pub id: u64,
    pub draw_b_spline_polygon: bool,
}

impl Command<SetBezierC2DrawBSplinePolygon> for SetBezierC2DrawBSplinePolygon {
    fn execute(command: &SetBezierC2DrawBSplinePolygon, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend.storage.beziers_c2.get_mut(&command.id).unwrap();
        bezier.set_draw_b_spline_polygon(command.draw_b_spline_polygon);
        let event = Rc::new(BezierC2DrawBSplinePolygonSet::new(bezier.id, bezier.draw_b_spline_polygon));
        drop(backend);
        let backend = app_state.borrow();
        backend.services.event_publisher.publish(event);
    }
}
