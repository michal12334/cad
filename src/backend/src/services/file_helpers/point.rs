use serde::{Deserialize, Serialize};

use crate::services::file_helpers::xyz::Xyz;

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub id: u64,
    pub name: String,
    pub position: Xyz,
}
