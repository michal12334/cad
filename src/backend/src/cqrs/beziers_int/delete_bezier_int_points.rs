use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::bezier_int_points_deleted::BezierIntPointsDeleted;

pub struct DeleteBezierIntPoints {
    pub id: u64,
    pub points: Vec<u64>,
}

impl Command<DeleteBezierIntPoints> for DeleteBezierIntPoints {
    fn execute(command: &DeleteBezierIntPoints, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend.storage.beziers_int.get(&command.id).unwrap();
        let points: Vec<_> = bezier.points
            .iter()
            .filter(|p| !command.points.contains(&p.id))
            .map(|p| backend.storage.points.get(&p.id).unwrap().clone())
            .collect();
        let bezier = backend.storage.beziers_int.get_mut(&command.id).unwrap();
        bezier.update_points(points);
        drop(backend);
        let backend = app_state.borrow();
        let points_deleted = Rc::new(BezierIntPointsDeleted::new(
            command.id,
            command.points.clone(),
        ));
        backend.services.event_publisher.publish(points_deleted);
    }
}
