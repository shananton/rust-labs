use crate::geometry::Ray;
use crate::vector::{Float, Vec3f};

// The camera is always located at 0, facing -Z
pub struct Camera {
    half_width: Float,
    half_height: Float,
    z_dir: Float,
}

impl Camera {
    pub fn new(vertical_fov: Float, cols: usize, rows: usize) -> Self {
        Self {
            half_width: cols as Float / 2.0,
            half_height: rows as Float / 2.0,
            z_dir: -(rows as Float) / (2.0 * Float::tan(vertical_fov / 2.0)),
        }
    }

    pub fn get_ray_for_pixel(&self, col: usize, row: usize) -> Ray {
        let x_dir = col as Float + 0.5 - self.half_width;
        let y_dir = -(row as Float + 0.5 - self.half_height);
        let cast_ray_direction = Vec3f::new(x_dir, y_dir, self.z_dir);

        Ray::new(Vec3f::ZERO, cast_ray_direction)
    }
}