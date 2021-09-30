use crate::vector::{Float, Vec3f};

pub struct Light {
    position: Vec3f,
    intensity: Float,
}

impl Light {
    pub fn new(position: Vec3f, intensity: Float) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> Vec3f {
        self.position
    }

    pub fn intensity(&self) -> Float {
        self.intensity
    }
}
