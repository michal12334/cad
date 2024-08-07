use serde::{Deserialize, Serialize};
use crate::services::file_helpers::geometry_obj::GeometryObj;
use crate::services::file_helpers::point::Point;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub points: Vec<Point>,
    pub geometry: Vec<GeometryObj>,
}
