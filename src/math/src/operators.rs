use std::ops::{Add, Mul, Sub};

use crate::matrix4::Matrix4;
use crate::vector3::Vector3;
use crate::vector4::Vector4;

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vector4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Matrix4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Matrix4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[0][j] * rhs.data[i][0]
                    + self.data[1][j] * rhs.data[i][1]
                    + self.data[2][j] * rhs.data[i][2]
                    + self.data[3][j] * rhs.data[i][3];
            }
        }
        result
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.data[0][0] * rhs.x
                + self.data[1][0] * rhs.y
                + self.data[2][0] * rhs.z
                + self.data[3][0] * rhs.w,
            y: self.data[0][1] * rhs.x
                + self.data[1][1] * rhs.y
                + self.data[2][1] * rhs.z
                + self.data[3][1] * rhs.w,
            z: self.data[0][2] * rhs.x
                + self.data[1][2] * rhs.y
                + self.data[2][2] * rhs.z
                + self.data[3][2] * rhs.w,
            w: self.data[0][3] * rhs.x
                + self.data[1][3] * rhs.y
                + self.data[2][3] * rhs.z
                + self.data[3][3] * rhs.w,
        }
    }
}

impl Mul<Matrix4> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Matrix4) -> Vector4 {
        Vector4 {
            x: self.x * rhs.data[0][0]
                + self.y * rhs.data[0][1]
                + self.z * rhs.data[0][2]
                + self.w * rhs.data[0][3],
            y: self.x * rhs.data[1][0]
                + self.y * rhs.data[1][1]
                + self.z * rhs.data[1][2]
                + self.w * rhs.data[1][3],
            z: self.x * rhs.data[2][0]
                + self.y * rhs.data[2][1]
                + self.z * rhs.data[2][2]
                + self.w * rhs.data[2][3],
            w: self.x * rhs.data[3][0]
                + self.y * rhs.data[3][1]
                + self.z * rhs.data[3][2]
                + self.w * rhs.data[3][3],
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}
