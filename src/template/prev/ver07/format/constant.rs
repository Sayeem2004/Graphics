// Imports
use crate::template::prev::ver07::format::{matrix::Matrix, pixel::Pixel};
use lazy_static::lazy_static;

/// Common color constants
pub const AQUA_PIXEL: Pixel = Pixel::new_value(0, 255, 255);
pub const BLACK_PIXEL: Pixel = Pixel::new_value(0, 0, 0);
pub const LIME_PIXEL: Pixel = Pixel::new_value(0, 255, 0);
pub const RED_PIXEL: Pixel = Pixel::new_value(255, 0, 0);
pub const WHITE_PIXEL: Pixel = Pixel::new_value(255, 255, 255);
pub const YELLOW_PIXEL: Pixel = Pixel::new_value(255, 255, 0);

lazy_static! {
    /// Hermite matrix
    pub static ref HERMITE: Matrix = Matrix {
        row_num: 4,
        col_num: 4,
        data: vec![
            vec![2.00, -3.00, 0.00, 1.00],
            vec![-2.00, 3.00, 0.00, 0.00],
            vec![1.00, -2.00, 1.00, 0.00],
            vec![1.00, -1.00, 0.00, 0.00]
        ]
    };

    /// Bezier matrix
    pub static ref BEZIER: Matrix = Matrix {
        row_num: 4,
        col_num: 4,
        data: vec![
            vec![-1.00, 3.00, -3.00, 1.00],
            vec![3.00, -6.00, 3.00, 0.00],
            vec![-3.00, 3.00, 0.00, 0.00],
            vec![1.00, 0.00, 0.00, 0.00]
        ]
    };
}

lazy_static! {
    /// View vectors
    pub static ref ZVIEW: Vec<f32> = vec![0.0, 0.0, 1.0];
    pub static ref YVIEW: Vec<f32> = vec![0.0, 1.0, 0.0];
    pub static ref XVIEW: Vec<f32> = vec![1.0, 0.0, 0.0];
}
