use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend::cqrs::cqrs::CQRS;
use backend::cqrs::surfaces_c2::surface_c2_points::SurfaceC2Points;
use backend_events::surfaces_c2::surface_c2_points_selected::SurfaceC2PointsSelected;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::object_id::ObjectId;
use crate::ui::Ui;

pub struct SelectedSurfaceC2PointsOnSurfaceC2PointsSelected {
    pub ui: Rc<RefCell<Ui>>,
    pub cqrs: CQRS,
}

impl Consumer<SurfaceC2PointsSelected> for SelectedSurfaceC2PointsOnSurfaceC2PointsSelected {
    fn consume(&self, message: &SurfaceC2PointsSelected) {
        let points = self.cqrs.get(&SurfaceC2Points {
            id: message.surface_id,
        });
        self.ui
            .borrow_mut()
            .selected_objects
            .extend(points.iter().map(|p| ObjectId::Point(p.id)));
    }
}

impl AnyConsumer for SelectedSurfaceC2PointsOnSurfaceC2PointsSelected {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
