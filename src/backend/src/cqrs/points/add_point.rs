use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::points::point_created::PointCreated;
use crate::domain::point::Point;
use crate::domain::transformer::LittleTransformer;

pub struct AddPoint {
    pub id: u64,
}

impl Command<AddPoint> for AddPoint {
    fn execute(command: &AddPoint, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let point = Point::new(
            command.id,
            LittleTransformer::from_cursor(&backend.storage.cursor),
        );
        let point_name = point.name.clone();
        backend.storage.points.insert(command.id, point);
        drop(backend);
        let backend = app_state.borrow();
        let point_created = Rc::new(PointCreated::new(command.id, point_name));
        backend.services.event_publisher.publish(point_created);
    }
}
