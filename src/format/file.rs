// Imports
use std::fs;
use std::io::{BufWriter, Write};
use crate::format::image;

// Function for creating an image file
pub fn create_file(path : &str, img : image::Image) {
    // Creating path and file
    let file = fs::File::create(path).expect("Unable to create file");

    // Writing data
    let mut writer = BufWriter::new(file);
    writer.write_all(format!("{}", img).as_bytes()).expect("Unable to write data");

    // Printing path
    println!("Image file is named {}", path);
}

#[allow(dead_code)]
// Checking if a certain file exists
pub fn file_exists(path : &str) -> bool {
    return fs::metadata(path).is_ok();
}
