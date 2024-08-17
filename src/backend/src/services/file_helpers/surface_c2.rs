use serde::{Deserialize, Serialize};
use crate::services::file_helpers::xyz::Xyu32;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceC2 {
    pub id: u64,
    pub name: String,
    pub patches: Vec<SurfaceC2Patch>,
    pub size: Xyu32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceC2Patch {
    pub id: u64,
    pub object_type: String,
    pub name: String,
    pub control_points: Vec<SurfaceC2ControlPoint>,
    pub samples: Xyu32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceC2ControlPoint {
    pub id: u64,
}
