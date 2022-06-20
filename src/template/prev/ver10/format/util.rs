// Imports
use crate::template::prev::ver10::format::image::Image;
use std::io::{BufWriter, Write};
use std::process::Command;
use std::{fs, fs::File};

/// Function for creating a ppm ascii file
pub fn create_ppm_ascii(path: &str, img: &Image, mode: i32) {
    // Attempting to create file
    let file: File = fs::File::create(path).expect("Unable to create file");

    // Attempting to create writer
    let mut writer: BufWriter<File> = BufWriter::new(file);

    // Writing information
    writer
        .write_all(format!("{}", img).as_bytes())
        .expect("Unable to write data");

    // Ending message
    if (mode == 0) {
        println!("Image file is named {}", path);
    }
}

/// Checking if a certain file exists
pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

/// Function that displays an image file using built in open command
pub fn open_image(path: &str) {
    Command::new("display")
        .arg(path)
        .status()
        .expect("Open command failed to run");
}
