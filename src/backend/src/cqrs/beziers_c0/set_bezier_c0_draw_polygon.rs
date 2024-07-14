use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::beziers_c0::bezier_c0_draw_polygon_set::BezierC0DrawPolygonSet;

pub struct SetBezierC0DrawPolygon {
    pub id: u64,
    pub draw_polygon: bool,
}

impl Command<SetBezierC0DrawPolygon> for SetBezierC0DrawPolygon {
    fn execute(command: &SetBezierC0DrawPolygon, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend.storage.beziers_c0.get_mut(&command.id).unwrap();
        bezier.set_draw_polygon(command.draw_polygon);
        let event = Rc::new(BezierC0DrawPolygonSet::new(bezier.id, bezier.draw_polygon));
        drop(backend);
        let backend = app_state.borrow();
        backend.services.event_publisher.publish(event);
    }
}
