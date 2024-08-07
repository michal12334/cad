use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BezierInt {
    pub id: u64,
    pub name: String,
    pub control_points: Vec<BezierIntPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BezierIntPoint {
    pub id: u64,
}
