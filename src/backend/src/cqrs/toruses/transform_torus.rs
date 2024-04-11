use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::cqrs::toruses::torus_details::TransformerDTO;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TransformTours {
    pub id: u64,
    pub transformer: TransformerDTO,
}

impl Command<TransformTours> for TransformTours {
    fn execute(command: &TransformTours, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        let torus = app_state.storage.toruses.get_mut(&command.id).unwrap();
        torus.transform(
            command.transformer.position,
            command.transformer.rotation,
            command.transformer.scale,
        );
    }
}
