// Imports
use crate::compiler;

/// Function that creates all images from work 11
pub fn create_work11_images(mode: i32) {
    // Creating donut animation gif
    compiler::compile("data/w11/w11_simple_50.mdl", mode);
}
