use crate::ray::Ray;

pub trait Object {
  fn intersect(&self, ray: &Ray) -> bool;
}