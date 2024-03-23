use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;
use crate::domain::selected_object::SelectedObject;

pub struct SelectObjects {
    pub objects: Vec<u64>,
}

impl Command<SelectObjects> for SelectObjects {
    fn execute(command: &SelectObjects, app_state: &mut AppState) {
        app_state.storage.selected_objects = command.objects
            .iter()
            .map(|&id| SelectedObject::new(id))
            .collect();
    }
}
