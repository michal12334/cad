use backend::cqrs::{
    common::new_id::NewId,
    cqrs::CQRS,
    intersections::find_intersection::{FindIntersection, IntersectionObjectIdDTO},
};
use egui::{Context, Widget};

use crate::object::Object;

use super::popup::Popup;

pub struct FindIntersectionPopup {
    is_closed: bool,
    ids: [IntersectionObjectIdDTO; 2],
    texture_size: usize,
}

impl FindIntersectionPopup {
    pub fn new(ids: [IntersectionObjectIdDTO; 2]) -> Self {
        Self {
            is_closed: false,
            ids,
            texture_size: 200,
        }
    }
}

impl Popup for FindIntersectionPopup {
    fn build(&mut self, cqrs: &mut CQRS, context: &Context) -> Vec<Object> {
        egui::Window::new("Find Intersection").show(context, |ui| {
            ui.horizontal(|ui| {
                egui::DragValue::new(&mut self.texture_size)
                    .clamp_range(100..=2000)
                    .ui(ui);
                ui.label("texture size");
            });

            ui.horizontal(|ui| {
                if ui.button("Find").clicked() {
                    let id = cqrs.handle(&NewId {});
                    cqrs.execute(&FindIntersection {
                        id1: self.ids[0],
                        id2: self.ids[1],
                        intersection_id: id,
                        texture_size: self.texture_size,
                    });

                    self.is_closed = true;
                }
                if ui.button("Close").clicked() {
                    self.is_closed = true;
                }
            });
        });

        vec![]
    }

    fn is_closed(&self) -> bool {
        self.is_closed
    }
}
