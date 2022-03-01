// Imports
use std::io::{BufWriter, Write};
use crate::format::image::Image;
use std::fs;

/// Function for creating a ppm ascii file
pub fn create_ppm_ascii(path : &str, img : Image) {
    // Attempting to create file
    let file = fs::File::create(path).expect("Unable to create file");

    // Attempting to create writer
    let mut writer = BufWriter::new(file);

    // Writing information
    writer.write_all(format!("{}", img).as_bytes()).expect("Unable to write data");

    // Ending message
    println!("Image file is named {}", path);
}

/// Checking if a certain file exists
pub fn file_exists(path : &str) -> bool {
    return fs::metadata(path).is_ok();
}
