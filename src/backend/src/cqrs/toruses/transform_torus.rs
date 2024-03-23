use crate::cqrs::cqrs::Command;
use crate::cqrs::toruses::torus_details::TransformerDTO;

pub struct TransformTours {
    pub id: u64,
    pub transformer: TransformerDTO,
}

impl Command<TransformTours> for TransformTours {
    fn execute(command: &TransformTours, app_state: &mut crate::app_state::AppState) {
        let torus = app_state.storage.toruses.get_mut(&command.id).unwrap();
        torus.transform(command.transformer.position, command.transformer.rotation, command.transformer.scale);
    }
}
