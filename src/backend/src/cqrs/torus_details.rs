use crate::app_state::AppState;
use crate::cqrs::cqrs::Query;

pub struct TorusDetails {}

pub struct TorusDTO {
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
    fn get(_query: &TorusDetails, app_state: &AppState) -> TorusDTO {
        TorusDTO {
            major_radius: app_state.torus.major_radius,
            minor_radius: app_state.torus.minor_radius,
            major_segments: app_state.torus.major_segments,
            minor_segments: app_state.torus.minor_segments,
            transformer: TransformerDTO {
                position: app_state.transformer.position,
                rotation: app_state.transformer.rotation,
                scale: app_state.transformer.scale,
            },
        }
    }
}
