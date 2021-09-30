use crate::vector::{Vec3f, Float, EPSILON};
use crate::geometry::{Shape, Ray};
use crate::render::object::{SceneObject, Material};

pub struct HorizontalCheckerboardFragment {
    y: Float,
    square_side: Float,
    is_even: bool,
    material: &'static Material,
}

impl HorizontalCheckerboardFragment {
    pub fn new(z: Float, square_side: Float, is_even: bool, material: &'static Material) -> Self {
        Self { y: z, square_side, is_even, material }
    }
}

impl Shape for HorizontalCheckerboardFragment {
    fn distance_to_intersection(&self, ray: &Ray) -> Option<Float> {
        let dir_y = ray.direction_normalized().y;
        if dir_y.abs() < EPSILON {
            return None;
        }
        let dist_to_intersection = (self.y - ray.origin().y) / dir_y;
        if dist_to_intersection < EPSILON {
            return None;
        }

        let intersection_point = ray.origin() + dist_to_intersection * ray.direction_normalized();

        let even_square = ((intersection_point.x / self.square_side).floor() as i32 +
            (intersection_point.z / self.square_side).floor() as i32) % 2 == 0;

        if even_square == self.is_even { Some(dist_to_intersection) } else { None }
    }

    fn normal_at(&self, _point: Vec3f) -> Vec3f {
        Vec3f::new(0.0, 1.0, 0.0)
    }
}

impl SceneObject for HorizontalCheckerboardFragment {
    fn material(&self) -> &Material {
        self.material
    }
}