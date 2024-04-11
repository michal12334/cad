use std::cell::RefCell;
use std::rc::Rc;
use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;
use crate::domain::selected_object::SelectedObject;

pub struct SelectObjects {
    pub objects: Vec<SelectionObjectDTO>,
}

#[derive(Clone, Copy)]
pub struct SelectionObjectDTO {
    pub id: u64,
    pub object_type: ObjectTypeDTO,
}

#[derive(Clone, Copy)]
pub enum ObjectTypeDTO {
    Torus,
    Point,
}

impl Command<SelectObjects> for SelectObjects {
    fn execute(command: &SelectObjects, app_state: Rc<RefCell<AppState>>) {
        let mut app_state = app_state.borrow_mut();
        app_state.storage.selected_objects = command
            .objects
            .iter()
            .map(|&obj| match obj.object_type {
                ObjectTypeDTO::Torus => SelectedObject::new_torus(obj.id),
                ObjectTypeDTO::Point => SelectedObject::new_point(obj.id),
            })
            .collect();
    }
}
