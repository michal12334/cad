use crate::cqrs::cqrs::Query;
use crate::cqrs::toruses::torus_details::{TorusDTO, TransformerDTO};

pub struct AllToruses;

impl Query<AllToruses, Vec<TorusDTO>> for AllToruses {
    fn get(_query: &AllToruses, app_state: &crate::app_state::AppState) -> Vec<TorusDTO> {
        app_state.storage.toruses.values()
            .map(|torus| TorusDTO {
                id: torus.id,
                name: torus.name.clone(),
                major_radius: torus.major_radius,
                minor_radius: torus.minor_radius,
                major_segments: torus.major_segments,
                minor_segments: torus.minor_segments,
                transformer: TransformerDTO {
                    position: torus.transformer.position,
                    rotation: torus.transformer.rotation,
                    scale: torus.transformer.scale,
                },
            })
            .collect()
    }
}
