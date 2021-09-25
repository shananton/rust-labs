use super::{Camera, Light};
use crate::render::object::{Ball, SceneObject, Material};
use crate::vector::{Vec3f, Float};
use crate::geometry::{Ray, Shape};


pub struct Scene {
    balls: Vec<Ball>,
    lights: Vec<Light>,
    background_color: Vec3f,
    camera: Camera,
}

impl Scene {
    pub fn new(background_color: Vec3f, camera: Camera) -> Self {
        Self {
            balls: Vec::new(),
            lights: Vec::new(),
            background_color,
            camera,
        }
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn get_color_of_pixel(&self, col: usize, row: usize) -> Vec3f {
        let cast_ray = self.camera.get_ray_for_pixel(col, row);
        self.get_color_of_ray(&cast_ray)
    }

    fn calculate_color(&self, point: Vec3f, view_direction: Vec3f, surface_normal: Vec3f, material: &Material) -> Vec3f {
        let unit_normal = surface_normal.normalized();
        self.diffuse_light_intensity(point, unit_normal) * material.diffuse_weight() * material.diffuse_color() +
            self.specular_light_intensity(point, unit_normal, view_direction, material.specular_exponent())
                * material.specular_weight() * Vec3f::new(1.0, 1.0, 1.0)
    }

    fn get_color_of_ray(&self, ray: &Ray) -> Vec3f {
        match self.intersect_with_first_object(&ray) {
            None => { self.background_color }
            Some((dist, object)) => {
                let material = object.material();
                let intersection_point = ray.origin() + dist * ray.direction_normalized();
                let unit_normal = object.normal_at(intersection_point).normalized();

                let mut diffuse_light_intensity = 0.0;
                let mut specular_light_intensity = 0.0;

                for light in &self.lights {
                    let light_dir_normalized =
                        (light.position() - intersection_point).normalized();

                    diffuse_light_intensity += light.intensity() * light_dir_normalized.dot(unit_normal).max(0.0);
                    specular_light_intensity += light.intensity() *
                        (-light_dir_normalized).reflected(unit_normal)
                            .dot(ray.direction_normalized()).max(0.0).powf(material.specular_exponent());
                }

                material.diffuse_weight() * diffuse_light_intensity * material.diffuse_color() +
                    material.specular_weight() * specular_light_intensity * Vec3f::ONE
            }
        }
    }

    fn intersect_with_first_object(&self, ray: &Ray) -> Option<(Float, &dyn SceneObject)> {
        self.balls.iter()
            .filter_map(|b| b.distance_to_intersection(&ray)
                .map(|c| (c, b)))
            .min_by(|(c1, _), (c2, _)| c1.partial_cmp(c2).unwrap())
            .map(|(c, b)| (c, b as &dyn SceneObject))
    }

    fn diffuse_light_intensity(&self, point: Vec3f, unit_normal: Vec3f) -> Float {
        self.lights.iter()
            .map(|l| Scene::diffuse_single_light_intensity(l, point, unit_normal))
            .sum()
    }

    fn specular_light_intensity(&self, point: Vec3f, unit_normal: Vec3f, view_direction: Vec3f, specular_exponent: Float) -> Float {
        self.lights.iter()
            .map(|l| Scene::specular_single_light_intensity(l, point, unit_normal, view_direction, specular_exponent))
            .sum()
    }

    fn diffuse_single_light_intensity(light: &Light, point: Vec3f, unit_normal: Vec3f) -> Float {
        light.intensity() * unit_normal.dot(light.position() - point).max(0.0)
    }

    fn specular_single_light_intensity(light: &Light, point: Vec3f, unit_normal: Vec3f, view_direction: Vec3f, specular_exponent: Float) -> Float {
        light.intensity() * (point - light.position()).reflected(unit_normal).dot(view_direction).max(0.0).powf(specular_exponent)
    }
}