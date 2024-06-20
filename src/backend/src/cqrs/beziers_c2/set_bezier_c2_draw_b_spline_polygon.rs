use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;

pub struct SetBezierC2DrawBSplinePolygon {
    pub id: u64,
    pub draw_b_spline_polygon: bool,
}

impl Command<SetBezierC2DrawBSplinePolygon> for SetBezierC2DrawBSplinePolygon {
    fn execute(command: &SetBezierC2DrawBSplinePolygon, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let bezier = app_state.storage.beziers_c2.get_mut(&command.id).unwrap();
        bezier.set_draw_b_spline_polygon(command.draw_b_spline_polygon);
    }
}
