use serde::{Deserialize, Serialize};
use crate::services::file_helpers::xyz::{Xyu32, Xyz};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Torus {
    pub id: u64,
    pub name: String,
    pub small_radius: f64,
    pub large_radius: f64,
    pub position: Xyz,
    pub rotation: Xyz,
    pub scale: Xyz,
    pub samples: Xyu32,
}
