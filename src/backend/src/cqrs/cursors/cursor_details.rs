use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::LittleTransformerDTO;

pub struct CursorDetails;

pub struct CursorDTO {
    pub name: String,
    pub transformer: LittleTransformerDTO,
}

impl Query<CursorDetails, CursorDTO> for CursorDetails {
    fn get(_query: &CursorDetails, app_state: &crate::app_state::AppState) -> CursorDTO {
        let cursor = &app_state.storage.cursor;
        CursorDTO {
            name: cursor.name.clone(),
            transformer: LittleTransformerDTO {
                position: cursor.transformer.position,
            },
        }
    }
}
