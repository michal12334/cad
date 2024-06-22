use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::bezier_c2_draw_bernstein_points_set::BezierC2DrawBernsteinPointsSet;

pub struct SetBezierC2DrawBernsteinPoints {
    pub id: u64,
    pub draw_bernstein_points: bool,
}

impl Command<SetBezierC2DrawBernsteinPoints> for SetBezierC2DrawBernsteinPoints {
    fn execute(command: &SetBezierC2DrawBernsteinPoints, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend.storage.beziers_c2.get_mut(&command.id).unwrap();
        bezier.set_draw_bernstein_points(command.draw_bernstein_points);
        backend.services.event_publisher.publish(Rc::new(BezierC2DrawBernsteinPointsSet::new(bezier.id, bezier.draw_bernstein_points)));
    }
}
