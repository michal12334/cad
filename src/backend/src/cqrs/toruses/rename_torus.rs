use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;

pub struct RenameTorus {
    pub id: u64,
    pub name: String,
}

impl Command<RenameTorus> for RenameTorus {
    fn execute(command: &RenameTorus, app_state: &mut AppState) {
        let torus = app_state.storage.toruses.get_mut(&command.id).unwrap();
        torus.rename(&command.name);
    }
}
