use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Query;

pub struct SurfaceC0Details {
    pub id: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceC0DTO {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
    pub tess_level: u8,
    pub size: (u32, u32),
    pub is_cylinder: bool,
}

impl Query<SurfaceC0Details, SurfaceC0DTO> for SurfaceC0Details {
    fn get(query: &SurfaceC0Details, app_state: Rc<RefCell<Backend>>) -> SurfaceC0DTO {
        let app_state = app_state.borrow();
        let surface_c0 = app_state.storage.surfaces_c0.get(&query.id).unwrap();
        SurfaceC0DTO {
            id: surface_c0.id,
            name: surface_c0.name.clone(),
            draw_polygon: surface_c0.draw_polygon,
            tess_level: surface_c0.tess_level,
            size: surface_c0.size,
            is_cylinder: surface_c0.is_cylinder,
        }
    }
}
