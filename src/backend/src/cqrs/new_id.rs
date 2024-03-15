use crate::app_state::AppState;
use crate::cqrs::cqrs::{Operation, Query};

pub struct NewId;

impl Operation<NewId, u64> for NewId {
    fn handle(query: &NewId, app_state: &mut AppState) -> u64 {
        app_state.services.id_generator.next()
    }
}
