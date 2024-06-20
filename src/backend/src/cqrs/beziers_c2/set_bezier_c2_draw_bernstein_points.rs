use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;

pub struct SetBezierC2DrawBernsteinPoints {
    pub id: u64,
    pub draw_bernstein_points: bool,
}

impl Command<SetBezierC2DrawBernsteinPoints> for SetBezierC2DrawBernsteinPoints {
    fn execute(command: &SetBezierC2DrawBernsteinPoints, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let bezier = app_state.storage.beziers_c2.get_mut(&command.id).unwrap();
        bezier.set_draw_bernstein_points(command.draw_bernstein_points);
    }
}
