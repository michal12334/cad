use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;

pub struct UpdateTransformer {
    pub position: (f64, f64, f64),
    pub rotation: (f64, f64, f64),
    pub scale: (f64, f64, f64),
}

impl Command<UpdateTransformer> for UpdateTransformer {
    fn execute(command: &UpdateTransformer, app_state: &mut AppState) {
        app_state.transformer.position = command.position;
        app_state.transformer.rotation = command.rotation;
        app_state.transformer.scale = command.scale;
    }
}
