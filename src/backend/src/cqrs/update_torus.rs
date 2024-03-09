use crate::cqrs::cqrs::Command;
use crate::domain::mesh::Mesh;

pub struct UpdateTorus {
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
}

impl Command<UpdateTorus> for UpdateTorus {
    fn execute(command: &UpdateTorus, app_state: &mut crate::app_state::AppState) {
        app_state.torus.major_radius = command.major_radius;
        app_state.torus.minor_radius = command.minor_radius;
        app_state.torus.major_segments = command.major_segments;
        app_state.torus.minor_segments = command.minor_segments;
        app_state.mesh = Mesh::from_torus(&app_state.torus);
    }
}
