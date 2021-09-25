pub mod material;
pub mod ball;

pub use ball::*;
pub use material::*;

use crate::geometry::Shape;

pub trait SceneObject: Shape {
    fn material(&self) -> &Material;
}