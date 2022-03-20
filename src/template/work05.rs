// Imports
use crate::format::parse;
use std::fs;

/// Function that creates all images from work 05
pub fn create_work05_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating shapes image test
    parse::parse("data/w05/w05_script", 500, mode);

    // Creating solar system image
    parse::parse("data/w05/w05_solar", 750, mode);
}
