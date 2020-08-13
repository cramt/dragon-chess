use std::ops;
use std::fmt;

#[derive(Copy, Clone, Debug, std::cmp::Eq, std::hash::Hash)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3 { x: 0, y: 0, z: 0 }
    }

    pub fn down() -> Vector3 {
        Vector3 { x: 0, y: 0, z: -1 }
    }

    pub fn forward() -> Vector3 {
        Vector3 { x: 0, y: 1, z: 0 }
    }

    pub fn up() -> Vector3 {
        Vector3 { x: 0, y: 0, z: 1 }
    }

    pub fn lf() -> Vector3 {
        Vector3 { x: 1, y: 1, z: 0 }
    }

    pub fn min() -> Vector3 {
        Vector3::new(i32::min_value(), i32::min_value(), i32::min_value())
    }

    pub fn to_vec(self) -> Vec<i32> {
        vec![self.x, self.y, self.z]
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Mul<i32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i32) -> Self::Output {
        let v = &rhs;
        Vector3 { x: self.x * v, y: self.y * v, z: self.z * v }
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = &format! {"x: {},y: {},z: {}", self.x, self.y, self.z}[..];
        fmt.write_str(str)?;
        Ok(())
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

