use crate::vector::{Vec3f, Float};

pub struct Material {
    diffuse_color: Vec3f,
    specular_exponent: Float,
    diffuse_weight: Float,
    specular_weight: Float,
}

impl Material {
    pub const fn new(diffuse_color: Vec3f, specular_exponent: Float,
                     diffuse_weight: Float, specular_weight: Float) -> Self {
        Self {
            diffuse_color, specular_exponent, diffuse_weight, specular_weight
        }
    }

    pub fn diffuse_color(&self) -> Vec3f {
        self.diffuse_color
    }

    pub fn specular_exponent(&self) -> Float {
        self.specular_exponent
    }

    pub fn diffuse_weight(&self) -> Float {
        self.diffuse_weight
    }

    pub fn specular_weight(&self) -> Float {
        self.specular_weight
    }
}