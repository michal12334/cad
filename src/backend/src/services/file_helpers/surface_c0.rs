use serde::{Deserialize, Serialize};
use crate::services::file_helpers::xyz::Xyu32;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceC0 {
    pub id: u64,
    pub name: String,
    pub patches: Vec<SurfaceC0Patch>,
    pub size: Xyu32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceC0Patch {
    pub id: u64,
    pub object_type: String,
    pub name: String,
    pub control_points: Vec<SurfaceC0ControlPoint>,
    pub samples: Xyu32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceC0ControlPoint {
    pub id: u64,
}
