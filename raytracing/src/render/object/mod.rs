pub mod material;
pub mod ball;
pub mod plane;

pub use ball::*;
pub use material::*;
pub use plane::*;

use crate::geometry::Shape;

pub trait SceneObject: Shape {
    fn material(&self) -> &Material;
}