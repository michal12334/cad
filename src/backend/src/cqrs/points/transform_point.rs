use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::cqrs::points::point_details::LittleTransformerDTO;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TransformPoint {
    pub id: u64,
    pub transformer: LittleTransformerDTO,
}

impl Command<TransformPoint> for TransformPoint {
    fn execute(command: &TransformPoint, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let point = app_state.storage.points.get_mut(&command.id).unwrap();
        point.transform(command.transformer.position);
    }
}
