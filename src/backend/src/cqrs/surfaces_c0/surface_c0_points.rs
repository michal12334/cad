use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::{LittleTransformerDTO, PointDTO};

pub struct SurfaceC0Points {
    pub id: u64,
}

impl Query<SurfaceC0Points, Vec<PointDTO>> for SurfaceC0Points {
    fn get(query: &SurfaceC0Points, app_state: Rc<RefCell<Backend>>) -> Vec<PointDTO> {
        let app_state = app_state.borrow();
        let surface_c0 = app_state.storage.surfaces_c0.get(&query.id).unwrap();
        surface_c0
            .points
            .iter()
            .map(|bp| {
                let p = app_state.storage.points.get(&bp.id).unwrap();
                PointDTO {
                    id: p.id,
                    name: p.name.clone(),
                    transformer: LittleTransformerDTO {
                        position: p.transformer.position,
                    },
                }
            })
            .collect()
    }
}
