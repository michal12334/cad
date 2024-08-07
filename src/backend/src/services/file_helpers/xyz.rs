use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Xyu32 {
    pub x: u32,
    pub y: u32,
}
