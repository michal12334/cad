use std::cell::RefCell;
use std::rc::Rc;
use crate::app_state::AppState;
use crate::cqrs::cqrs::Operation;

pub struct NewId;

impl Operation<NewId, u64> for NewId {
    fn handle(_query: &NewId, app_state: Rc<RefCell<AppState>>) -> u64 {
        app_state.borrow_mut().services.id_generator.next()
    }
}
