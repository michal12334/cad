use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::{LittleTransformerDTO, PointDTO};

pub struct AllPoints;

impl Query<AllPoints, Vec<PointDTO>> for AllPoints {
    fn get(_query: &AllPoints, app_state: &crate::app_state::AppState) -> Vec<PointDTO> {
        app_state.storage.points.values()
            .map(|point| PointDTO {
                id: point.id,
                name: point.name.clone(),
                transformer: LittleTransformerDTO {
                    position: point.transformer.position,
                },
            })
            .collect()
    }
}
