// Imports
use crate::template::prev::ver09::format::{matrix::Matrix, pixel::Pixel};
use lazy_static::lazy_static;

/// Common color constants
pub const BLACK_PIXEL: Pixel = Pixel::new_value(0, 0, 0);
pub const WHITE_PIXEL: Pixel = Pixel::new_value(255, 255, 255);

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

/// View vectors
pub const ZVIEW: (f32, f32, f32) = (0.0, 0.0, 1.0);

// Common lighting divisions and specular lighting cosine power
pub const EQV: (f32, f32, f32, f32, f32, f32, f32, f32, f32) =
    (0.33, 0.33, 0.33, 0.33, 0.33, 0.33, 0.33, 0.33, 0.33);
pub const EXP: f32 = 3_f32;
