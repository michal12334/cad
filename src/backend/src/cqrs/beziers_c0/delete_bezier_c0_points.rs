use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::bezier_c0_points_deleted::BezierC0PointsDeleted;

pub struct DeleteBezierC0Points {
    pub id: u64,
    pub points: Vec<u64>,
}

impl Command<DeleteBezierC0Points> for DeleteBezierC0Points {
    fn execute(command: &DeleteBezierC0Points, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier_c0 = backend.storage.beziers_c0.get_mut(&command.id).unwrap();
        bezier_c0.delete_points(&command.points);
        drop(backend);
        let backend = app_state.borrow();
        let points_deleted = Rc::new(BezierC0PointsDeleted::new(command.id, command.points.clone()));
        backend.services.event_publisher.publish(points_deleted);
    }
}
