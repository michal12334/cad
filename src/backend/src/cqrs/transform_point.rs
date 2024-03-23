use crate::cqrs::cqrs::Command;
use crate::cqrs::point_details::LittleTransformerDTO;

pub struct TransformPoint {
    pub id: u64,
    pub transformer: LittleTransformerDTO,
}

impl Command<TransformPoint> for TransformPoint {
    fn execute(command: &TransformPoint, app_state: &mut crate::app_state::AppState) {
        let point = app_state.storage.points.get_mut(&command.id).unwrap();
        point.transform(command.transformer.position);
    }
}
