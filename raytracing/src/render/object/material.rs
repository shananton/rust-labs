use crate::vector::{Float, Vec3f};

pub struct Material {
    diffuse_color: Vec3f,
    specular_exponent: Float,
    refractive_index: Float,
    albedo: Albedo,
}

pub struct Albedo {
    pub diffuse: Float,
    pub specular: Float,
    pub reflect: Float,
    pub refract: Float,
}

impl Material {
    pub const fn new(
        diffuse_color: Vec3f,
        specular_exponent: Float,
        refractive_index: Float,
        albedo: Albedo,
    ) -> Self {
        Self {
            diffuse_color,
            specular_exponent,
            refractive_index,
            albedo,
        }
    }

    pub fn diffuse_color(&self) -> Vec3f {
        self.diffuse_color
    }

    pub fn specular_exponent(&self) -> Float {
        self.specular_exponent
    }

    pub fn refractive_index(&self) -> Float {
        self.refractive_index
    }

    pub fn diffuse_weight(&self) -> Float {
        self.albedo.diffuse
    }

    pub fn specular_weight(&self) -> Float {
        self.albedo.specular
    }

    pub fn reflect_weight(&self) -> Float {
        self.albedo.reflect
    }

    pub fn refract_weight(&self) -> Float {
        self.albedo.refract
    }
}
