// Imports
use crate::compiler;

/// Function that creates all images from work 12
pub fn create_work12_images(mode: i32) {
    // Creating savecs test image
    compiler::compile("data/mdl/w12_savecs_test.mdl", mode);

    // Creating lighting test image
    compiler::compile("data/mdl/w12_lighting_test.mdl", mode);

    // Creating knobs test image
    compiler::compile("data/mdl/w12_knobs_test.mdl", mode);

    // Creating perlin noise image
    compiler::compile("data/mdl/w12_terrain.mdl", mode);

    // Cleaning some things up
    if (crate::format::util::file_exists("temp")) {
        std::fs::remove_dir_all("temp").expect("Unable to remove temp directory");
    }
}
