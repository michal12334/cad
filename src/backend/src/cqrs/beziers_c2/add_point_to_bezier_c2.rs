use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::points::point_added_to_bezier_c2::PointAddedToBezierC2;

pub struct AddPointToBezierC2 {
    pub id: u64,
    pub point_id: u64,
}

impl Command<AddPointToBezierC2> for AddPointToBezierC2 {
    fn execute(command: &AddPointToBezierC2, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend.storage.beziers_c2.get(&command.id).unwrap();
        let points: Vec<_> = bezier
            .b_spline_points
            .iter()
            .map(|p| backend.storage.points.get(&p.id).unwrap().clone())
            .collect();
        let mut points = points;
        let point = backend.storage.points.get(&command.point_id).unwrap();
        let point_name = point.name.clone();
        points.push(point.clone());
        let bezier = backend.storage.beziers_c2.get_mut(&command.id).unwrap();
        bezier.update_points(points);
        drop(backend);
        let backend = app_state.borrow();
        let point_added_to_bezier = Rc::new(PointAddedToBezierC2::new(
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
