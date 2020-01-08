use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;

pub struct Sphere {
  pub pos: Point,
  pub radius: f64,
}

impl Object for Sphere {
  fn intersect(&self, ray: &Ray) -> bool {
    let oc = ray.pos.sub(&self.pos);
    let a = ray.dir.dot(&ray.dir);
    let b = 2.0 * oc.dot(&ray.dir);
    let c = oc.dot(&oc) - self.radius*self.radius;
    let dis = b*b - 4.0*a*c;
    return dis > 0.0;
  }
}