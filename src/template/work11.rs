// Imports
use crate::compiler;

/// Function that creates all images from work 11
pub fn create_work11_images(mode: i32) {
    // Creating donut animation gif
    compiler::compile("data/mdl/w11_simple_300.mdl", mode);

    // Creating spiral animation gif
    compiler::compile("data/mdl/w11_spiral.mdl", mode);

    // Creating wave animation gif
    compiler::compile("data/mdl/w11_wave.mdl", mode);

    // Creating trophy animation gif
    compiler::compile("data/mdl/w11_trophy.mdl", mode);
}
