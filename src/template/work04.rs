// Imports
use crate::format::parse;
use std::fs;

/// Function that creates all images from work 04
pub fn create_work04_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating test image
    parse::parse("data/w04/w04_script", 750);
}
