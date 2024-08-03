use serde::{Deserialize, Serialize};
use crate::services::file_helpers::point::Point;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub points: Vec<Point>,
}
