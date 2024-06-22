use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::bezier_c2_draw_bernstein_polygon_set::BezierC2DrawBernsteinPolygonSet;

pub struct SetBezierC2DrawBernsteinPolygon {
    pub id: u64,
    pub draw_bernstein_polygon: bool,
}

impl Command<SetBezierC2DrawBernsteinPolygon> for SetBezierC2DrawBernsteinPolygon {
    fn execute(command: &SetBezierC2DrawBernsteinPolygon, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend.storage.beziers_c2.get_mut(&command.id).unwrap();
        bezier.set_draw_bernstein_polygon(command.draw_bernstein_polygon);
        let event = Rc::new(BezierC2DrawBernsteinPolygonSet::new(bezier.id, bezier.draw_bernstein_polygon));
        drop(backend);
        let backend = app_state.borrow();
        backend.services.event_publisher.publish(event);
    }
}
