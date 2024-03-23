use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;

pub struct RenamePoint {
    pub id: u64,
    pub name: String,
}

impl Command<RenamePoint> for RenamePoint {
    fn execute(command: &RenamePoint, app_state: &mut AppState) {
        let point = app_state.storage.points.get_mut(&command.id).unwrap();
        point.rename(&command.name);
    }
}
