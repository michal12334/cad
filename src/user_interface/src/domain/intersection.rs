use std::sync::Arc;

use bit_vec::BitVec;
use egui::{ColorImage, ImageData, TextureHandle};

pub struct Intersection {
    pub id: u64,
    pub name: String,
    pub uv_texture_handle: Option<TextureHandle>,
    pub uv_texture: ImageData,
    pub st_texture_handle: Option<TextureHandle>,
    pub st_texture: ImageData,
}

impl Intersection {
    pub fn get_texture(t: &[BitVec]) -> ImageData {
        let mut data = Vec::new();
        for i in 0..t.len() {
            for j in 0..t[i].len() {
                if t[i][j] {
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
        ImageData::Color(Arc::new(ColorImage::from_rgba_unmultiplied(
            [200, 200],
            &data,
        )))
    }
}
