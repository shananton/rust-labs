use crate::geometry::Ray;
use crate::render::object::{Ball, HorizontalCheckerboardFragment, SceneObject};
use crate::vector::{Float, Vec3f};

use super::{Camera, Light};

pub struct Scene {
    balls: Vec<Ball>,
    checkerboard_fragments: Vec<HorizontalCheckerboardFragment>,
    lights: Vec<Light>,
    background_color: Vec3f,
    camera: Camera,
    raycasting_recursion_depth: u32,
}

impl Scene {
    pub fn new(background_color: Vec3f, camera: Camera, raycasting_recursion_depth: u32) -> Self {
        Self {
            balls: Vec::new(),
            checkerboard_fragments: Vec::new(),
            lights: Vec::new(),
            background_color,
            camera,
            raycasting_recursion_depth,
        }
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    pub fn add_checkerboard_fragment(&mut self, fragment: HorizontalCheckerboardFragment) {
        self.checkerboard_fragments.push(fragment);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn get_color_of_pixel(&self, col: usize, row: usize) -> Vec3f {
        let cast_ray = self.camera.get_ray_for_pixel(col, row);
        self.get_color_of_ray(&cast_ray, self.raycasting_recursion_depth)
    }

    fn get_color_of_ray(&self, ray: &Ray, remaining_depth: u32) -> Vec3f {
        if remaining_depth == 0 {
            return self.background_color;
        }
        match self.intersect_with_first_object(ray) {
            None => { self.background_color }
            Some((dist, object)) => {
                let material = object.material();
                let intersection_point = ray.origin() + dist * ray.direction_normalized();
                let unit_normal = object.normal_at(intersection_point).normalized();

                let reflected_ray = Ray::new(intersection_point,
                                             Self::reflect(ray.direction_normalized(), unit_normal));
                let reflect_color = self.get_color_of_ray(&reflected_ray, remaining_depth - 1);
                let refracted_ray =
                    Self::refract(ray.direction_normalized(), unit_normal, 1.0 / material.refractive_index())
                        .map(|dir| Ray::new(intersection_point, dir));
                let refract_color = refracted_ray.map_or(Vec3f::ZERO, |ray| {
                    self.get_color_of_ray(&ray, remaining_depth - 1)
                });


                let mut diffuse_light_intensity = 0.0;
                let mut specular_light_intensity = 0.0;

                for light in &self.lights {
                    let light_dir_normalized =
                        (light.position() - intersection_point).normalized();

                    if self.intersect_with_first_object(&Ray::new(intersection_point, light_dir_normalized)).is_some() {
                        continue;
                    }

                    diffuse_light_intensity += light.intensity() * light_dir_normalized.dot(unit_normal).max(0.0);
                    specular_light_intensity += light.intensity() *
                        Self::reflect(light_dir_normalized, unit_normal)
                            .dot(ray.direction_normalized()).max(0.0).powf(material.specular_exponent());
                }

                material.diffuse_weight() * diffuse_light_intensity * material.diffuse_color() +
                    material.specular_weight() * specular_light_intensity * Vec3f::ONE +
                    material.reflect_weight() * reflect_color +
                    material.refract_weight() * refract_color
            }
        }
    }

    fn intersect_with_first_object(&self, ray: &Ray) -> Option<(Float, &dyn SceneObject)> {
        [
            self.intersect_with_type(ray, self.balls.iter()),
            self.intersect_with_type(ray, self.checkerboard_fragments.iter())
        ].iter().filter_map(|opt| *opt)
            .min_by(|(c1, _), (c2, _)| c1.partial_cmp(c2).unwrap())
    }

    fn intersect_with_type<'a, T: 'a + SceneObject>(&self, ray: &Ray, obj_iter: impl Iterator<Item=&'a T>) -> Option<(Float, &'a dyn SceneObject)> {
        obj_iter.filter_map(|b| b.distance_to_intersection(ray)
            .map(|c| (c, b)))
            .min_by(|(c1, _), (c2, _)| c1.partial_cmp(c2).unwrap())
            .map(|(c, o)| (c, o as &dyn SceneObject))
    }

    fn reflect(v_normalized: Vec3f, unit_normal: Vec3f) -> Vec3f {
        v_normalized - 2.0 * v_normalized.dot(unit_normal) * unit_normal
    }

    fn refract(v_normalized: Vec3f, unit_normal: Vec3f, eta: Float) -> Option<Vec3f> {
        let cos_i = -v_normalized.dot(unit_normal).clamp(-1.0, 1.0);
        if cos_i < 0.0 {
            return Self::refract(v_normalized, -unit_normal, 1.0 / eta);
        }
        let k = 1.0 - eta * eta * (1.0 - cos_i * cos_i);
        if k < 0.0 { None } else { Some(eta * v_normalized + (eta * cos_i - k.sqrt()) * unit_normal) }
    }
}