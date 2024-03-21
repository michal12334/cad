use crate::app_state::AppState;
use crate::cqrs::cqrs::Command;
use crate::domain::point::Point;

pub struct AddPoint {
    pub id: u64,
}

impl Command<AddPoint> for AddPoint {
    fn execute(command: &AddPoint, app_state: &mut AppState) {
        let point = Point::new(command.id);
        app_state.storage.points.insert(command.id, point);
    }
}
