use std::cell::RefCell;
use std::rc::Rc;

use itertools::Itertools;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::surfaces_c0::surface_c0_points_selected::SurfaceC0PointsSelected;
use crate::domain::selected_object::SelectedObject;

pub struct SelectSurfaceC0Points {
    pub surface_id: u64,
}

impl Command<SelectSurfaceC0Points> for SelectSurfaceC0Points {
    fn execute(command: &SelectSurfaceC0Points, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let surface = backend
            .storage
            .surfaces_c0
            .get(&command.surface_id)
            .unwrap();
        let points = surface.points.clone();
        let event = Rc::new(SurfaceC0PointsSelected::new(surface.id));
        backend.storage.selected_objects.extend(
            points
                .iter()
                .unique_by(|p| p.id)
                .map(|p| SelectedObject::new_point(p.id)),
        );
        drop(backend);
        let backend = app_state.borrow();
        backend.services.event_publisher.publish(event);
    }
}
