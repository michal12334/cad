use std::cell::RefCell;
use std::rc::Rc;
use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;
use crate::cqrs::points::point_details::LittleTransformerDTO;

pub struct TransformCursor {
    pub transformer: LittleTransformerDTO,
}

impl Command<TransformCursor> for TransformCursor {
    fn execute(command: &TransformCursor, app_state: Rc<RefCell<AppState>>) {
        let mut app_state = app_state.borrow_mut();
        app_state
            .storage
            .cursor
            .transform(command.transformer.position);
    }
}
