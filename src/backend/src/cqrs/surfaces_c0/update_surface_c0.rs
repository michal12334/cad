use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::surfaces_c0::surface_c0_updated::SurfaceC0Updated;

pub struct UpdateSurfaceC0 {
    pub id: u64,
    pub draw_polygon: bool,
    pub tess_level: u8,
}

impl Command<UpdateSurfaceC0> for UpdateSurfaceC0 {
    fn execute(command: &UpdateSurfaceC0, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let surface_c0 = backend.storage.surfaces_c0.get_mut(&command.id).unwrap();
        surface_c0.set_draw_polygon(command.draw_polygon);
        surface_c0.set_tess_level(command.tess_level);
        drop(backend);
        let backend = app_state.borrow();
        backend
            .services
            .event_publisher
            .publish(Rc::new(SurfaceC0Updated::new(
                command.id,
                command.draw_polygon,
                command.tess_level,
            )));
    }
}
