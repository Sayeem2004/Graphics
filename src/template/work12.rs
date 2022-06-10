// Imports
use crate::compiler;

/// Function that creates all images from work 12
pub fn create_work12_images(mode: i32) {
    // Creating test image
    compiler::compile("data/w12/w12_savecs_test.mdl", mode);
}
