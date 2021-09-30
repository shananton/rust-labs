use super::{Ray, Shape};
use crate::vector::{Float, Vec3f, EPSILON};

pub struct Sphere {
    center: Vec3f,
    radius: Float,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: Float) -> Self {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    fn distance_to_intersection(&self, ray: &Ray) -> Option<Float> {
        let orig_to_center = self.center - ray.origin();
        let ray_dir_normalized = ray.direction_normalized();

        // Let P be the point on the line (containing the ray) that is closest to the sphere
        let orig_to_p_dist = orig_to_center.dot(ray_dir_normalized);

        let dist_from_center_to_ray_squared =
            orig_to_center.norm_squared() - orig_to_p_dist * orig_to_p_dist;
        let p_to_surf_dist_squared = self.radius * self.radius - dist_from_center_to_ray_squared;
        if p_to_surf_dist_squared < 0.0 {
            return None;
        }

        let p_to_surf_dist = p_to_surf_dist_squared.sqrt();
        let mut result = orig_to_p_dist - p_to_surf_dist;
        if result < EPSILON {
            result = orig_to_p_dist + p_to_surf_dist;
        }
        if result < EPSILON {
            None
        } else {
            Some(result)
        }
    }

    fn normal_at(&self, point: Vec3f) -> Vec3f {
        point - self.center
    }
}
