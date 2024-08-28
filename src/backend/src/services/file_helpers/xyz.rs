use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_tuple(tuple: (f64, f64, f64)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Xyu32 {
    pub x: u32,
    pub y: u32,
}
