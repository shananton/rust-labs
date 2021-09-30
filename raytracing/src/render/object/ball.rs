use crate::geometry;
use crate::vector::{Float, Vec3f};
use crate::geometry::{Shape, Ray};
use crate::render::object::{SceneObject, Material};

pub struct Ball {
    shape: geometry::Sphere,
    material: &'static Material,
}

impl Ball {
    pub fn new(shape: geometry::Sphere, material: &'static Material) -> Self {
        Self { shape, material }
    }
}

impl Shape for Ball {
    fn distance_to_intersection(&self, ray: &Ray) -> Option<Float> {
        self.shape.distance_to_intersection(ray)
    }

    fn normal_at(&self, point: Vec3f) -> Vec3f {
        self.shape.normal_at(point)
    }
}

impl SceneObject for Ball {
    fn material(&self) -> &Material {
        self.material
    }
}