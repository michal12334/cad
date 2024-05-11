use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::bezier_c0::BezierC0Point;
use crate::domain::events::point_added_to_bezier_c0::PointAddedToBezierC0;

pub struct AddPointToBezierC0 {
    pub id: u64,
    pub point_id: u64,
}

impl Command<AddPointToBezierC0> for AddPointToBezierC0 {
    fn execute(command: &AddPointToBezierC0, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend.storage.beziers_c0.get_mut(&command.id).unwrap();
        bezier.add_point(BezierC0Point {
            id: command.point_id,
        });
        let point = backend.storage.points.get(&command.point_id).unwrap();
        let point_name = point.name.clone();
        drop(backend);
        let backend = app_state.borrow();
        let point_added_to_bezier = Rc::new(PointAddedToBezierC0::new(
            command.point_id,
            command.id,
            point_name,
        ));
        backend
            .services
            .event_publisher
            .publish(point_added_to_bezier);
    }
}
