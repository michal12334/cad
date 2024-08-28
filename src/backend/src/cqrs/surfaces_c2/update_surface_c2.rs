use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::surfaces_c2::surface_c2_updated::SurfaceC2Updated;

pub struct UpdateSurfaceC2 {
    pub id: u64,
    pub draw_polygon: bool,
    pub tess_level: u8,
}

impl Command<UpdateSurfaceC2> for UpdateSurfaceC2 {
    fn execute(command: &UpdateSurfaceC2, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let surface_c2 = backend.storage.surfaces_c2.get_mut(&command.id).unwrap();
        surface_c2.set_draw_polygon(command.draw_polygon);
        surface_c2.set_tess_level(command.tess_level);
        drop(backend);
        let backend = app_state.borrow();
        backend
            .services
            .event_publisher
            .publish(Rc::new(SurfaceC2Updated::new(
                command.id,
                command.draw_polygon,
                command.tess_level,
            )));
    }
}
