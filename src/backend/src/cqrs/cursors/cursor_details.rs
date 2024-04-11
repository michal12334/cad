use crate::backend::Backend;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::LittleTransformerDTO;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CursorDetails;

pub struct CursorDTO {
    pub name: String,
    pub transformer: LittleTransformerDTO,
}

impl Query<CursorDetails, CursorDTO> for CursorDetails {
    fn get(_query: &CursorDetails, app_state: Rc<RefCell<Backend>>) -> CursorDTO {
        let app_state = app_state.borrow();
        let cursor = &app_state.storage.cursor;
        CursorDTO {
            name: cursor.name.clone(),
            transformer: LittleTransformerDTO {
                position: cursor.transformer.position,
            },
        }
    }
}
