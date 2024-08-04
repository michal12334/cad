use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use backend::cqrs::beziers_c0::all_beziers_c0::AllBeziersC0;
use backend::cqrs::beziers_c2::all_beziers_c2::AllBeziersC2;
use backend::cqrs::beziers_int::all_beziers_int::AllBeziersInt;
use backend::cqrs::beziers_c0::bezier_c0_points::BezierC0Points;
use backend::cqrs::beziers_c2::bezier_c2_b_spline_points::BezierC2BSplinePoints;
use backend::cqrs::beziers_c2::bezier_c2_bernstein_points::BezierC2BernsteinPoints;
use backend::cqrs::beziers_int::bezier_int_bernstein_points::BezierIntBernsteinPoints;
use backend::cqrs::cqrs::CQRS;
use backend_events::common::scene_loaded::SceneLoaded;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::drawing::domain::bezier_c0::BezierC0;
use crate::drawing::domain::bezier_c2::BezierC2;
use crate::drawing::domain::bezier_int::BezierInt;
use crate::drawing::drawing_storage::DrawingStorage;

pub struct RebuildStorageOnSceneLoaded {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<SceneLoaded> for RebuildStorageOnSceneLoaded {
    fn consume(&self, event: &SceneLoaded) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage.beziers_c0.clear();
        drawing_storage.beziers_c2.clear();
        drawing_storage.beziers_int.clear();
        drawing_storage.surfaces_c0.clear();
        drawing_storage.surfaces_c2.clear();
        
        for bezier_c0 in self.cqrs.get(&AllBeziersC0 {}) {
            let points = self.cqrs.get(&BezierC0Points { id: bezier_c0.id });
            drawing_storage
                .beziers_c0
                .insert(bezier_c0.id, BezierC0::new(bezier_c0.id, &points, &self.display));
        }
        
        for bezier_c2 in self.cqrs.get(&AllBeziersC2 {}) {
            let bernstein_points = self.cqrs.get(&BezierC2BernsteinPoints { id: bezier_c2.id });
            let b_spline_points = self.cqrs.get(&BezierC2BSplinePoints { id: bezier_c2.id });
            drawing_storage.beziers_c2.insert(
                bezier_c2.id,
                BezierC2::new(bezier_c2.id, &bernstein_points, &b_spline_points, &self.display),
            );
        }

        for bezier_int in self.cqrs.get(&AllBeziersInt {}) {
            let points = self.cqrs.get(&BezierIntBernsteinPoints { id: bezier_int.id });
            drawing_storage
                .beziers_int
                .insert(bezier_int.id, BezierInt::new(bezier_int.id, &points, &self.display));
        }
    }
}

impl AnyConsumer for RebuildStorageOnSceneLoaded {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
