// Imports
use crate::script::parse;
use std::fs;

/// Function that creates all images from work 09
pub fn create_work09_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating face image test
    parse::parse("data/w09/w09_face", 500, mode);

    // Creating robot image test
    parse::parse("data/w09/w09_robot", 500, mode);

    // Creating shapes image test
    parse::parse("data/w09/w09_shapes", 500, mode);
}
