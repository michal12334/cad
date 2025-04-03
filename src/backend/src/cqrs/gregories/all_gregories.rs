use std::{cell::RefCell, rc::Rc};

use crate::{backend::Backend, cqrs::cqrs::Query};

use super::gregory_details::GregoryDTO;

pub struct AllGregories;

impl Query<AllGregories, Vec<GregoryDTO>> for AllGregories {
    fn get(_: &AllGregories, app_state: Rc<RefCell<Backend>>) -> Vec<GregoryDTO> {
        let app_state = app_state.borrow();
        app_state
            .storage
            .gregories
            .values()
            .map(|g| GregoryDTO {
                id: g.id,
                name: g.name.clone(),
                tess_level: g.tess_level,
                points: g
                    .patches
                    .iter()
                    .flat_map(|p| {
                        p.top
                            .iter()
                            .chain(p.top_sides.iter())
                            .chain(p.bottom_sides.iter())
                            .chain(p.bottom.iter())
                            .chain(p.u_inner.iter())
                            .chain(p.v_inner.iter())
                    })
                    .cloned()
                    .collect(),
            })
            .collect()
    }
}
