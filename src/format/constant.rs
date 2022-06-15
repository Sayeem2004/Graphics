// Imports
use crate::format::{matrix::Matrix, pixel::Pixel};
use lazy_static::lazy_static;

/// Common color constants
pub const AQUA_PIXEL: Pixel = Pixel::new_value(0, 255, 255);
pub const BLACK_PIXEL: Pixel = Pixel::new_value(0, 0, 0);
pub const BLUE_PIXEL: Pixel = Pixel::new_value(0, 0, 255);
pub const FUCHSIA_PIXEL: Pixel = Pixel::new_value(255, 0, 255);
pub const GRAY_PIXEL: Pixel = Pixel::new_value(128, 128, 128);
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

/// Difference vectors
pub const DX: [i32; 4] = [-1, 0, 1, 0];
pub const DY: [i32; 4] = [0, 1, 0, -1];

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
pub const YVIEW: (f32, f32, f32) = (0.0, 1.0, 0.0);
pub const XVIEW: (f32, f32, f32) = (1.0, 0.0, 0.0);

/// Common lighting divisions
pub const AMBDIV: (f32, f32, f32, f32, f32, f32, f32, f32, f32) =
    (1.00, 1.00, 1.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00);
pub const DIFDIV: (f32, f32, f32, f32, f32, f32, f32, f32, f32) =
    (0.00, 0.00, 0.00, 1.00, 1.00, 1.00, 0.00, 0.00, 0.00);
pub const SPCDIV: (f32, f32, f32, f32, f32, f32, f32, f32, f32) =
    (0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 1.00, 1.00, 1.00);
pub const EQVDIV: (f32, f32, f32, f32, f32, f32, f32, f32, f32) =
    (0.33, 0.33, 0.33, 0.33, 0.33, 0.33, 0.33, 0.33, 0.33);

// Specular lighting cosine power
pub const EXP: f32 = 3_f32;

lazy_static! {
    /// Default ambient light value
    pub static ref AMB: Pixel = Pixel::new_scale(0.5);

    /// Default point light source
    pub static ref PNT: (Pixel, f32, f32, f32) = (Pixel::new_scale(1.0), 750.0, 750.0, 750.0);

    /// Default list of point light sources
    pub static ref PNTS: Vec<(Pixel, f32, f32, f32)> = vec![(Pixel::new_scale(1.0), 750.0, 750.0, 750.0)];
}

/// List of all possible script commands
pub const CMDS: [&str; 33] = [
    "alterlight",
    "ambient",
    "basename",
    "bezier",
    "box",
    "camera",
    "circle",
    "clear",
    "constants",
    "display",
    "focal",
    "frames",
    "hermite",
    "light",
    "line",
    "mesh",
    "move",
    "movelight",
    "pop",
    "push",
    "rotate",
    "save",
    "savecs",
    "saveknobs",
    "scale",
    "set",
    "setknobs",
    "shading",
    "sphere",
    "terrain",
    "torus",
    "tween",
    "vary",
];
