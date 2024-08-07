use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::services::file_helpers::bezier_c2::BezierC2;
use crate::services::file_helpers::torus::Torus;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "objectType", rename_all = "camelCase")]
pub enum GeometryObj {
    Torus(Torus),
    BezierC2(BezierC2),
}
