use bitflags::bitflags;
use std::{cell::RefCell, rc::Rc};

use crate::{
    backend::Backend,
    cqrs::cqrs::Command,
    domain::{
        events::intersections::intersection_textures_draw_set::IntersectionTexturesDrawSet,
        intersection::TextureDraw,
    },
};

pub struct SetIntersectionTexturesDraw {
    pub intersection_id: u64,
    pub uv_draw: TextureDrawDTO,
    pub st_draw: TextureDrawDTO,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextureDrawDTO: u32 {
        const True = 0b00000001;
        const False = 0b00000010;

        const Both = Self::True.bits() | Self::False.bits();
    }
}

impl Command<SetIntersectionTexturesDraw> for SetIntersectionTexturesDraw {
    fn execute(command: &SetIntersectionTexturesDraw, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let intersection = backend
            .storage
            .intersections
            .get_mut(&command.intersection_id)
            .unwrap();
        intersection.set_uv_draw(TextureDraw::from_bits(command.uv_draw.bits()).unwrap());
        intersection.set_st_draw(TextureDraw::from_bits(command.st_draw.bits()).unwrap());
        let event = IntersectionTexturesDrawSet::new(
            intersection.id,
            intersection.uv_draw,
            intersection.st_draw,
        );
        drop(backend);
        let backend = app_state.borrow();
        backend.services.event_publisher.publish(Rc::new(event));
    }
}
