pub mod sphere;
pub mod ray;

pub use sphere::*;
pub use ray::*;

use crate::vector::{Float, Vec3f};

pub trait Shape {
    fn distance_to_intersection(&self, ray: &Ray) -> Option<Float>;
    fn normal_at(&self, point: Vec3f) -> Vec3f;
}