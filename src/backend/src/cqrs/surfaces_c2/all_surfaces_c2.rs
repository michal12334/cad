use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Query;
use crate::cqrs::surfaces_c2::surface_c2_details::SurfaceC2DTO;

pub struct AllSurfacesC2;

impl Query<AllSurfacesC2, Vec<SurfaceC2DTO>> for AllSurfacesC2 {
    fn get(query: &AllSurfacesC2, app_state: Rc<RefCell<Backend>>) -> Vec<SurfaceC2DTO> {
        let app_state = app_state.borrow();
        app_state
            .storage
            .surfaces_c2
            .values()
            .map(|surface_c2| SurfaceC2DTO {
                id: surface_c2.id,
                name: surface_c2.name.clone(),
                draw_polygon: surface_c2.draw_polygon,
                tess_level: surface_c2.tess_level,
                size: surface_c2.size,
                is_cylinder: surface_c2.is_cylinder,
            })
            .collect()
    }
}
