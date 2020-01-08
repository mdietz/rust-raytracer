extern crate bmp;

mod object;
mod point;
mod ray;
mod scene;
mod sphere;
mod vector;

use bmp::{Image, Pixel, px};
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::vector::Vector3;

struct Camera {
  pos: Point,
  dir: Vector3,
  fov: f64,
}

fn deg2rad(deg: f64) -> f64 {
  return deg/360.0 * 2.0 * std::f64::consts::PI;
}

fn main() {
  let width = 1028 as f64;
  let height = 1028 as f64;
  let fov = 90 as f64;
  let aspect_ratio = width/height as f64;
  let scale = deg2rad(fov * 0.5).tan();

  let camera = Camera {pos: Point {x: 0.0, y: 0.0, z: 0.0}, dir: Vector3 {x: 0.0, y: 0.0, z: -1.0}, fov: 90.0};
  let sphere = Sphere {pos: Point {x: 0.0, y: 0.0, z: -10.0}, radius: 5.0};
  let mut scene = Scene {objects: Vec::new()};
  scene.objects.push(Box::new(sphere));

  let mut img = Image::new(width as u32, height as u32);

  for j in 0..height as u32 {
    for i in 0..width as u32 {
      let j = j as f64;
      let i = i as f64;
      let x = (2.0 * (i + 0.5) / width - 1.0) * aspect_ratio * scale; 
      let y = (1.0 - 2.0 * (j + 0.5) / height) * scale;
      let dir = Vector3 { x: x, y: y, z: -1.0}.normalize();
      let ray = Ray {pos: Point {x: 0.0, y: 0.0 , z: 0.0}, dir: dir};
      for obj in scene.objects.iter() {
        if obj.intersect(&ray) {
          img.set_pixel(i as u32, j as u32, px!(255,255,255));
        } else {
          img.set_pixel(i as u32, j as u32, px!(0,0,0));
        }
      }
    } 
  }
  let _ = img.save("out.bmp");
}