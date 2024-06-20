use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::bezier_c2_points_deleted::BezierC2PointsDeleted;

pub struct DeleteBezierC2Points {
    pub id: u64,
    pub points: Vec<u64>,
}

impl Command<DeleteBezierC2Points> for DeleteBezierC2Points {
    fn execute(command: &DeleteBezierC2Points, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend.storage.beziers_c2.get(&command.id).unwrap();
        let points: Vec<_> = bezier.b_spline_points
            .iter()
            .filter(|p| !command.points.contains(&p.id))
            .map(|p| backend.storage.points.get(&p.id).unwrap().clone())
            .collect();
        let bezier = backend.storage.beziers_c2.get_mut(&command.id).unwrap();
        bezier.update_points(points);
        drop(backend);
        let backend = app_state.borrow();
        let points_deleted = Rc::new(BezierC2PointsDeleted::new(
            command.id,
            command.points.clone(),
        ));
        backend.services.event_publisher.publish(points_deleted);
    }
}
