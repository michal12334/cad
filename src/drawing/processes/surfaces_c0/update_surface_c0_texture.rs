use std::{any::Any, cell::RefCell, rc::Rc};

use backend::cqrs::intersections::intersection_object_texture::IntersectionObjectTexture;
use backend_events::intersections::intersection_textures_draw_set::IntersectionObjectIdDTO;
use glium::glutin::surface::WindowSurface;
use glium::{Display, Rect, Texture2d};

use backend::cqrs::cqrs::CQRS;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateSurfaceC0Texture {
    pub id: u64,
}

pub struct UpdateSurfaceC0TextureConsumer {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<UpdateSurfaceC0Texture> for UpdateSurfaceC0TextureConsumer {
    fn consume(&self, message: &UpdateSurfaceC0Texture) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let surface = drawing_storage.surfaces_c0.get_mut(&message.id).unwrap();
        let texture_data = self.cqrs.get(&IntersectionObjectTexture {
            id: IntersectionObjectIdDTO::SurfaceC0(message.id),
        });

        let texture = Texture2d::empty_with_format(
            &*self.display,
            glium::texture::UncompressedFloatFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            texture_data.len() as u32,
            texture_data.len() as u32,
        )
        .unwrap();

        texture.write(
            Rect {
                left: 0,
                bottom: 0,
                width: texture_data.len() as u32,
                height: texture_data.len() as u32,
            },
            texture_data.clone(),
        );

        surface.update_texture(texture);
    }
}

impl AnyConsumer for UpdateSurfaceC0TextureConsumer {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
