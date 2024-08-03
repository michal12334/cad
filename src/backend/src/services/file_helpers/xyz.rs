use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
