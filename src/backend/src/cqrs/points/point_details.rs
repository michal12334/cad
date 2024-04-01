use crate::app_state::AppState;
use crate::cqrs::cqrs::Query;

pub struct PointDetails {
    pub id: u64,
}

#[derive(Clone)]
pub struct PointDTO {
    pub id: u64,
    pub name: String,
    pub transformer: LittleTransformerDTO,
}

#[derive(Clone)]
pub struct LittleTransformerDTO {
    pub position: (f64, f64, f64),
}

impl Query<PointDetails, PointDTO> for PointDetails {
    fn get(query: &PointDetails, app_state: &AppState) -> PointDTO {
        let point = app_state.storage.points.get(&query.id).unwrap();
        PointDTO {
            id: point.id,
            name: point.name.clone(),
            transformer: LittleTransformerDTO {
                position: point.transformer.position,
            },
        }
    }
}
