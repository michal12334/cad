use crate::app_state::AppState;
use crate::cqrs::cqrs::Query;

pub struct TorusDetails {
    pub id: u64,
}

pub struct TorusDTO {
    pub id: u64,
    pub name: String,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
    pub transformer: TransformerDTO,
}

pub struct TransformerDTO {
    pub position: (f64, f64, f64),
    pub rotation: (f64, f64, f64),
    pub scale: (f64, f64, f64),
}

impl Query<TorusDetails, TorusDTO> for TorusDetails {
    fn get(query: &TorusDetails, app_state: &AppState) -> TorusDTO {
        let torus = app_state.storage.toruses.get(&query.id).unwrap();
        TorusDTO {
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
        }
    }
}
