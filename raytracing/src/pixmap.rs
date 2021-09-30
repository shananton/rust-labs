use crate::vector::{Float, Vec3f};

pub struct Pixmap {
    data: Vec<Vec3f>,
    cols: usize,
}

impl Pixmap {
    pub fn new(cols: usize, rows: usize) -> Self {
        Pixmap {
            data: vec![Default::default(); cols * rows],
            cols,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut result = vec![0; 3 * self.data.len()];
        let max_byte = u8::MAX as Float;
        for (i, v) in self.data.iter().enumerate() {
            result[3 * i] = (max_byte * v.x.clamp(0.0, 1.0)).round() as u8;
            result[3 * i + 1] = (max_byte * v.y.clamp(0.0, 1.0)).round() as u8;
            result[3 * i + 2] = (max_byte * v.z.clamp(0.0, 1.0)).round() as u8;
        }
        result
    }

    pub fn get(&self, col: usize, row: usize) -> &Vec3f {
        &self.data[row * self.cols + col]
    }

    pub fn get_mut(&mut self, col: usize, row: usize) -> &mut Vec3f {
        &mut self.data[row * self.cols + col]
    }
}
