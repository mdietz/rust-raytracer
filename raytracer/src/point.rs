use crate::vector::Vector3;

pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Point {
  pub fn add(&self, other: &Point) -> Vector3 {
    Vector3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
  }

  pub fn sub(&self, other: &Point) -> Vector3 {
    Vector3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
  }
}