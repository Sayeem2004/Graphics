// Imports
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::format::image;

// Function for creating an image file
pub fn create_file(name : &str, img : image::Image) {
    let file = File::create(name).expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    writer.write_all(format!("{}", img).as_bytes()).expect("Unable to write data");
    println!("Image file is named {}", name);
}
