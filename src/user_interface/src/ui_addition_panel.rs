use backend::cqrs::beziers_c0::add_bezier_c0::AddBezierC0;
use backend::cqrs::beziers_c0::bezier_c0_details::BezierC0Details;
use backend::cqrs::beziers_c2::add_bezier_c2::AddBezierC2;
use backend::cqrs::common::new_id::NewId;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::points::add_point::AddPoint;
use backend::cqrs::points::point_details::PointDetails;
use backend::cqrs::toruses::add_torus::AddTorus;
use backend::cqrs::toruses::torus_details::TorusDetails;

use crate::object::Object::{BeziersC0, Point, Torus};
use crate::ui::Ui;

type DomainBezier = crate::domain::bezier_c0::BezierC0;

impl Ui {
    pub fn build_object_addition_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
        if ui.button("Add Torus").clicked() {
            let id = cqrs.handle(&NewId {});
            cqrs.execute(&AddTorus {
                id,
                major_radius: 1.0,
                minor_radius: 0.5,
                major_segments: 100,
                minor_segments: 100,
            });
            self.objects.push(Torus(cqrs.get(&TorusDetails { id })));
        }
        if ui.button("Add Point").clicked() {
            let id = cqrs.handle(&NewId {});
            cqrs.execute(&AddPoint { id });
            self.objects.push(Point(cqrs.get(&PointDetails { id })));
        }
        if ui.button("Add Bezier C0").clicked() {
            let id = cqrs.handle(&NewId {});
            cqrs.execute(&AddBezierC0 { id });
            self.objects.push(BeziersC0(DomainBezier::from_dto(
                &cqrs.get(&BezierC0Details { id }),
            )));
        }
        if ui.button("Add Bezier C2").clicked() {
            let id = cqrs.handle(&NewId {});
            cqrs.execute(&AddBezierC2 { id });
        }
    }
}
