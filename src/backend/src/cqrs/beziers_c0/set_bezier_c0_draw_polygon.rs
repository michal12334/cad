use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;

pub struct SetBezierC0DrawPolygon {
    pub id: u64,
    pub draw_polygon: bool,
}

impl Command<SetBezierC0DrawPolygon> for SetBezierC0DrawPolygon {
    fn execute(command: &SetBezierC0DrawPolygon, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let bezier = app_state.storage.beziers_c0.get_mut(&command.id).unwrap();
        bezier.set_draw_polygon(command.draw_polygon);
    }
}
