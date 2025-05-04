use bit_vec::BitVec;
use math::vector3::Vector3;

use crate::cqrs::cqrs::Query;

use super::set_intersection_textures_draw::TextureDrawDTO;

pub struct AllIntersections;

pub struct IntersectionDTO {
    pub id: u64,
    pub name: String,
    pub uv_texture: Vec<BitVec>,
    pub st_texture: Vec<BitVec>,
    pub uv_draw: TextureDrawDTO,
    pub st_draw: TextureDrawDTO,
    pub points: Vec<Vector3>,
    pub wrap: bool,
}

impl Query<AllIntersections, Vec<IntersectionDTO>> for AllIntersections {
    fn get(
        _query: &AllIntersections,
        app_state: std::rc::Rc<std::cell::RefCell<crate::backend::Backend>>,
    ) -> Vec<IntersectionDTO> {
        let backend = app_state.borrow();
        backend
            .storage
            .intersections
            .values()
            .map(|intersection| IntersectionDTO {
                id: intersection.id,
                name: intersection.name.clone(),
                uv_texture: intersection.uv_texture.clone(),
                st_texture: intersection.st_texture.clone(),
                uv_draw: TextureDrawDTO::from_bits(intersection.uv_draw.bits()).unwrap(),
                st_draw: TextureDrawDTO::from_bits(intersection.st_draw.bits()).unwrap(),
                points: intersection.intersection_points.clone(),
                wrap: intersection.wrap,
            })
            .collect()
    }
}
