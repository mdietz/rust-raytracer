use crate::object::Object;

pub struct Scene {
  pub objects: Vec<Box<dyn Object>>,
}