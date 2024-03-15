use std::ffi::c_int;
use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;
use crate::domain::mesh::Mesh;

pub struct UpdateTorus {
    pub id: u64,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
}

impl Command<UpdateTorus> for UpdateTorus {
    fn execute(command: &UpdateTorus, app_state: &mut AppState) {
        if command.minor_radius >= command.major_radius { 
            return;
        }
        
        let torus = app_state.storage.toruses.get_mut(&command.id).unwrap();
        torus.update(command.major_radius, command.minor_radius, command.major_segments, command.minor_segments);
    }
}
