use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Query;
use crate::cqrs::surfaces_c0::surface_c0_details::SurfaceC0DTO;

pub struct AllSurfacesC0;

impl Query<AllSurfacesC0, Vec<SurfaceC0DTO>> for AllSurfacesC0 {
    fn get(_: &AllSurfacesC0, app_state: Rc<RefCell<Backend>>) -> Vec<SurfaceC0DTO> {
        let app_state = app_state.borrow();
        app_state
            .storage
            .surfaces_c0
            .values()
            .map(|surface_c0| SurfaceC0DTO {
                id: surface_c0.id,
                name: surface_c0.name.clone(),
                draw_polygon: surface_c0.draw_polygon,
                tess_level: surface_c0.tess_level,
                size: surface_c0.size,
                is_cylinder: surface_c0.is_cylinder,
            })
            .collect()
    }
}
