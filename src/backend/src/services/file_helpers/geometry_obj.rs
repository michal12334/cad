use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::services::file_helpers::bezier_c0::BezierC0;
use crate::services::file_helpers::bezier_c2::BezierC2;
use crate::services::file_helpers::bezier_int::BezierInt;
use crate::services::file_helpers::surface_c0::SurfaceC0;
use crate::services::file_helpers::torus::Torus;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "objectType", rename_all = "camelCase")]
pub enum GeometryObj {
    Torus(Torus),
    BezierC0(BezierC0),
    BezierC2(BezierC2),
    InterpolatedC2(BezierInt),
    BezierSurfaceC0(SurfaceC0),
}
