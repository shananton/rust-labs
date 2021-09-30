use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::{Display, Formatter};

pub type Float = f32;

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Display for Vec3f {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3f {
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Vec3f { x, y, z }
    }

    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);

    pub fn dot(self, rhs: Self) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vec3f::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn norm_squared(self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> Float {
        self.norm_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.norm()
    }
}

impl Neg for Vec3f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Float> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3f> for Float {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Self::Output {
        rhs * self
    }
}

impl Div<Float> for Vec3f {
    type Output = Self;

    fn div(self, rhs: Float) -> Self::Output {
        Vec3f::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}