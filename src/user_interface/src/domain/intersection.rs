use std::sync::Arc;

use bit_vec::BitVec;
use bitflags::bitflags;
use egui::{ColorImage, ImageData, TextureHandle};

pub struct Intersection {
    pub id: u64,
    pub name: String,
    pub uv_texture_handle: Option<TextureHandle>,
    pub uv_texture: ImageData,
    pub st_texture_handle: Option<TextureHandle>,
    pub st_texture: ImageData,
    pub uv_draw: TextureDraw,
    pub st_draw: TextureDraw,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextureDraw: u32 {
        const Red = 0b00000001;
        const Green = 0b00000010;

        const Both = Self::Red.bits() | Self::Green.bits();
    }
}

impl Intersection {
    pub fn get_texture(t: &[BitVec]) -> ImageData {
        let mut data = Vec::new();
        let ts = t.len();
        let s = 200usize;
        for i in 0..s {
            for j in 0..s {
                if t[i * ts / s][j * ts / s] {
                    data.push(255);
                    data.push(0);
                    data.push(0);
                    data.push(255);
                } else {
                    data.push(0);
                    data.push(255);
                    data.push(0);
                    data.push(255);
                }
            }
        }
        ImageData::Color(Arc::new(ColorImage::from_rgba_unmultiplied([s, s], &data)))
    }
}
