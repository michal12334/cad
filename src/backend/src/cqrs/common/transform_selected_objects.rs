use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;
use crate::cqrs::toruses::torus_details::TransformerDTO;

pub struct TransformSelectedObjects {
    pub transformer: TransformerDTO,
}

impl Command<TransformSelectedObjects> for TransformSelectedObjects {
    fn execute(command: &TransformSelectedObjects, app_state: &mut AppState) {
        for object in app_state.storage.selected_objects.iter() {
            if let Some(torus_id) = object.torus_id {
                let torus = app_state.storage.toruses.get_mut(&torus_id).unwrap();
                torus.transform(
                    command.transformer.position,
                    command.transformer.rotation,
                    command.transformer.scale,
                );
            }
            if let Some(point_id) = object.point_id {
                let point = app_state.storage.points.get_mut(&point_id).unwrap();
                point.transform(
                    command.transformer.position,
                );
            }
        }
    }
}
