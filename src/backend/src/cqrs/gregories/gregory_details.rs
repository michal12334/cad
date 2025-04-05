use std::{cell::RefCell, rc::Rc};

use math::vector3::Vector3;

use crate::{backend::Backend, cqrs::cqrs::Query};

pub struct GregoryDetails {
    pub gregory_id: u64,
}

pub struct GregoryDTO {
    pub id: u64,
    pub name: String,
    pub tess_level: u8,
    pub points: Vec<Vector3>,
    pub vectors: Vec<GregoryVectorDTO>,
    pub draw_vectors: bool,
}

pub struct GregoryVectorDTO {
    pub points: [Vector3; 2],
}

impl Query<GregoryDetails, GregoryDTO> for GregoryDetails {
    fn get(query: &GregoryDetails, app_state: Rc<RefCell<Backend>>) -> GregoryDTO {
        let gregory = &app_state.borrow().storage.gregories[&query.gregory_id];

        GregoryDTO {
            id: gregory.id,
            name: gregory.name.clone(),
            tess_level: gregory.tess_level,
            points: gregory
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
            vectors: gregory
                .vectors
                .iter()
                .map(|v| GregoryVectorDTO { points: v.points })
                .collect(),
            draw_vectors: gregory.draw_vectors,
        }
    }
}
