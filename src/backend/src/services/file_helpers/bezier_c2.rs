use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BezierC2 {
    pub id: u64,
    pub name: String,
    pub de_boor_points: Vec<BezierC2Point>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BezierC2Point {
    pub id: u64,
}
