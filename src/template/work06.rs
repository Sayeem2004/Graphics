// Imports
// use crate::format::{constant, matrix::Matrix, image::Image, file, pixel::Pixel};
use crate::script::parse;
// use crate::algorithm::shape;
use std::fs;

/// Function that creates all images from work 06
pub fn create_work06_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating shapes image test
    parse::parse("data/w06/w06_script", 500, mode);
}
