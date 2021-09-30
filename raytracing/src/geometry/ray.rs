use crate::vector::Vec3f;

pub struct Ray {
    origin: Vec3f,
    direction_normalized: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Self {
        Self {
            origin,
            direction_normalized: direction.normalized(),
        }
    }

    pub fn origin(&self) -> Vec3f {
        self.origin
    }

    pub fn direction_normalized(&self) -> Vec3f {
        self.direction_normalized
    }
}
