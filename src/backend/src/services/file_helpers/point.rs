use serde::{Deserialize, Serialize};
use crate::services::file_helpers::xyz::Xyz;

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub id: u64,
    pub name: String,
    pub position: Xyz,
}
