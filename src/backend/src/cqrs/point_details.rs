use crate::app_state::AppState;
use crate::cqrs::cqrs::Query;

pub struct PointDetails {
    pub id: u64,
}

pub struct PointDTO {
    pub id: u64,
    pub transformer: LittleTransformerDTO,
}

pub struct LittleTransformerDTO {
    pub position: (f64, f64, f64),
}

impl Query<PointDetails, PointDTO> for PointDetails {
    fn get(query: &PointDetails, app_state: &AppState) -> PointDTO {
        let point = app_state.storage.points.get(&query.id).unwrap();
        PointDTO {
            id: point.id,
            transformer: LittleTransformerDTO {
                position: point.transformer.position,
            }
        }
    }
}
