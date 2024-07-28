use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
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
    BezierC0,
    BezierC2,
    BezierInt,
    SurfaceC0,
    SurfaceC2,
}

impl Command<SelectObjects> for SelectObjects {
    fn execute(command: &SelectObjects, app_state: Rc<RefCell<Backend>>) {
        let mut app_state = app_state.borrow_mut();
        app_state.storage.selected_objects = command
            .objects
            .iter()
            .map(|&obj| match obj.object_type {
                ObjectTypeDTO::Torus => SelectedObject::new_torus(obj.id),
                ObjectTypeDTO::Point => SelectedObject::new_point(obj.id),
                ObjectTypeDTO::BezierC0 => SelectedObject::new_bezier_c0(obj.id),
                ObjectTypeDTO::BezierC2 => SelectedObject::new_bezier_c2(obj.id),
                ObjectTypeDTO::BezierInt => SelectedObject::new_bezier_int(obj.id),
                ObjectTypeDTO::SurfaceC0 => SelectedObject::new_surface_c0(obj.id),
                ObjectTypeDTO::SurfaceC2 => SelectedObject::new_surface_c2(obj.id),
            })
            .collect();
    }
}
