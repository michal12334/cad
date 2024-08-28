use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::{LittleTransformerDTO, PointDTO};

pub struct SurfaceC2Points {
    pub id: u64,
}

impl Query<SurfaceC2Points, Vec<PointDTO>> for SurfaceC2Points {
    fn get(query: &SurfaceC2Points, app_state: Rc<RefCell<Backend>>) -> Vec<PointDTO> {
        let app_state = app_state.borrow();
        let surface_c2 = app_state.storage.surfaces_c2.get(&query.id).unwrap();
        surface_c2
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
