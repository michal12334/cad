use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Query;

pub struct SurfaceC2Details {
    pub id: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceC2DTO {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
    pub tess_level: u8,
    pub size: (u32, u32),
}

impl Query<SurfaceC2Details, SurfaceC2DTO> for SurfaceC2Details {
    fn get(query: &SurfaceC2Details, app_state: Rc<RefCell<Backend>>) -> SurfaceC2DTO {
        let app_state = app_state.borrow();
        let surface_c2 = app_state.storage.surfaces_c2.get(&query.id).unwrap();
        SurfaceC2DTO {
            id: surface_c2.id,
            name: surface_c2.name.clone(),
            draw_polygon: surface_c2.draw_polygon,
            tess_level: surface_c2.tess_level,
            size: surface_c2.size,
        }
    }
}
