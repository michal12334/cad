use std::{cell::RefCell, rc::Rc};

use backend_events::intersections::intersection_textures_draw_set::IntersectionObjectIdDTO;

use crate::{
    backend::Backend,
    cqrs::cqrs::Query,
    domain::intersection::{IntersectionObjectId, TextureDraw},
};

pub struct IntersectionObjectTexture {
    pub id: IntersectionObjectIdDTO,
}

impl Query<IntersectionObjectTexture, Vec<Vec<f32>>> for IntersectionObjectTexture {
    fn get(query: &IntersectionObjectTexture, app_state: Rc<RefCell<Backend>>) -> Vec<Vec<f32>> {
        let backend = app_state.borrow();
        let domain_id = map_id(&query.id);
        let textures = backend
            .storage
            .intersections
            .values()
            .flat_map(|intersection| {
                [
                    if intersection.object1_id == domain_id {
                        Some((intersection.uv_texture.clone(), intersection.uv_draw))
                    } else {
                        None
                    },
                    if intersection.object2_id == domain_id {
                        Some((intersection.st_texture.clone(), intersection.st_draw))
                    } else {
                        None
                    },
                ]
            })
            .filter_map(|texture| texture)
            .collect::<Vec<_>>();
        let size = textures
            .iter()
            .map(|(texture, _)| texture.len())
            .max()
            .unwrap_or(0);
        let mut texture_data = vec![vec![1.0; size]; size];

        for i in 0..size {
            for j in 0..size {
                let mut value = 1.0;
                for (texture, draw) in textures.iter() {
                    let ts = texture.len();
                    if texture[i * ts / size][j * ts / size] && !draw.contains(TextureDraw::True) {
                        value = 0.0;
                    } else if !texture[i * ts / size][j * ts / size]
                        && !draw.contains(TextureDraw::False)
                    {
                        value = 0.0;
                    }
                }
                texture_data[i][j] = value;
            }
        }

        texture_data
    }
}

fn map_id(id: &IntersectionObjectIdDTO) -> IntersectionObjectId {
    match id {
        IntersectionObjectIdDTO::Torus(id) => IntersectionObjectId::Torus(*id),
        IntersectionObjectIdDTO::SurfaceC0(id) => IntersectionObjectId::SurfaceC0(*id),
        IntersectionObjectIdDTO::SurfaceC2(id) => IntersectionObjectId::SurfaceC2(*id),
    }
}
