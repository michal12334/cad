use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::surfaces_c0::surface_c0_points::SurfaceC0Points;
use backend_events::surfaces_c0::surface_c0_points_selected::SurfaceC0PointsSelected;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::object_id::ObjectId;
use crate::ui::Ui;

pub struct SelectedSurfaceC0PointsOnSurfaceC0PointsSelected {
    pub ui: Rc<RefCell<Ui>>,
    pub cqrs: CQRS,
}

impl Consumer<SurfaceC0PointsSelected> for SelectedSurfaceC0PointsOnSurfaceC0PointsSelected {
    fn consume(&self, message: &SurfaceC0PointsSelected) {
        let points = self.cqrs.get(&SurfaceC0Points { id: message.surface_id, });
        self.ui.borrow_mut().selected_objects.extend(points.iter().map(|p| ObjectId::Point(p.id)));
    }
}

impl AnyConsumer for SelectedSurfaceC0PointsOnSurfaceC0PointsSelected {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
