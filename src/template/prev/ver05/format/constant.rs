// Imports
use crate::template::prev::ver05::format::{matrix::Matrix, pixel::Pixel};
use lazy_static::lazy_static;

/// Common color constants
pub const AQUA_PIXEL: Pixel = Pixel::new_value(0, 255, 255);
pub const BLACK_PIXEL: Pixel = Pixel::new_value(0, 0, 0);
pub const BLUE_PIXEL: Pixel = Pixel::new_value(0, 0, 255);
pub const FUCHSIA_PIXEL: Pixel = Pixel::new_value(255, 0, 255);
pub const GREEN_PIXEL: Pixel = Pixel::new_value(0, 128, 0);
pub const LIME_PIXEL: Pixel = Pixel::new_value(0, 255, 0);
pub const MAROON_PIXEL: Pixel = Pixel::new_value(128, 0, 0);
pub const NAVY_PIXEL: Pixel = Pixel::new_value(0, 0, 128);
pub const OLIVE_PIXEL: Pixel = Pixel::new_value(128, 128, 0);
pub const PURPLE_PIXEL: Pixel = Pixel::new_value(128, 0, 128);
pub const RED_PIXEL: Pixel = Pixel::new_value(255, 0, 0);
pub const SILVER_PIXEL: Pixel = Pixel::new_value(192, 192, 192);
pub const TEAL_PIXEL: Pixel = Pixel::new_value(0, 128, 128);
pub const WHITE_PIXEL: Pixel = Pixel::new_value(255, 255, 255);
pub const YELLOW_PIXEL: Pixel = Pixel::new_value(255, 255, 0);

// Difference vectors
pub const DX: [i32; 4] = [-1, 0, 1, 0];
pub const DY: [i32; 4] = [0, 1, 0, -1];

// Hermite and bezier matrices
lazy_static! {
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
