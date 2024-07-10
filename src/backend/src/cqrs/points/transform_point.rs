use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::cqrs::points::point_details::LittleTransformerDTO;
use crate::domain::events::point_moved::PointMoved;

pub struct TransformPoint {
    pub id: u64,
    pub transformer: LittleTransformerDTO,
}

impl Command<TransformPoint> for TransformPoint {
    fn execute(command: &TransformPoint, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let point = backend.storage.points.get_mut(&command.id).unwrap();
        point.transform(command.transformer.position);
        drop(backend);
        let backend = app_state.borrow();
        backend
            .services
            .event_publisher
            .publish(Rc::new(PointMoved::new(
                command.id,
                command.transformer.position,
            )));
    }
}
