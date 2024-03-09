use crate::app_state::AppState;
use crate::cqrs::cqrs::Query;

pub struct TorusDetails {}

pub struct TorusDTO {
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
}

impl Query<TorusDetails, TorusDTO> for TorusDetails {
    fn get(_query: &TorusDetails, app_state: &AppState) -> TorusDTO {
        TorusDTO {
            major_radius: app_state.torus.major_radius,
            minor_radius: app_state.torus.minor_radius,
            major_segments: app_state.torus.major_segments,
            minor_segments: app_state.torus.minor_segments,
        }
    }
}
