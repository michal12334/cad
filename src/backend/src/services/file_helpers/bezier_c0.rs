use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BezierC0 {
    pub id: u64,
    pub name: String,
    pub control_points: Vec<BezierC0Point>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BezierC0Point {
    pub id: u64,
}
