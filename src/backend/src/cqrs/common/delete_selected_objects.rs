use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;

pub struct DeleteSelectedObjects;

impl Command<DeleteSelectedObjects> for DeleteSelectedObjects {
    fn execute(_command: &DeleteSelectedObjects, app_state: &mut AppState) {
        app_state.storage.toruses.retain(|_, torus| {
            !app_state.storage.selected_objects.iter().any(|object| object.torus_id == Some(torus.id))
        });
        app_state.storage.points.retain(|_, point| {
            !app_state.storage.selected_objects.iter().any(|object| object.point_id == Some(point.id))
        });
        app_state.storage.selected_objects.clear();
    }
}
