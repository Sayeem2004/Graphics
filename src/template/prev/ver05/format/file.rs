// Imports
use crate::template::prev::ver05::format::{image::Image, matrix::Matrix};
use std::io::{BufWriter, Write};
use std::process::Command;
use std::fs;

/// Function for creating a ppm ascii file
pub fn create_ppm_ascii(path: &str, img: &Image, mode: i32) {
    // Attempting to create file
    let file = fs::File::create(path).expect("Unable to create file");

    // Attempting to create writer
    let mut writer = BufWriter::new(file);

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

/// Parsing lines from a csv into a matrix
pub fn read_lines_csv(path: &str) -> Matrix {
    // Checking if file exists
    if (!file_exists(path)) {
        eprintln!("File {} not found, returning default value", path);
        return Matrix::new_matrix();
    }

    // Getting data from csv file and splitting it
    let data = fs::read_to_string(path).expect("Unable to read data");
    let lines = data.split('\n');

    // Creating edge matrix
    let mut mat = Matrix::new_matrix();

    // Iterating through data and adding to matrix
    for line in lines {
        // Splitting into numbers
        if (line.is_empty()) {
            continue;
        }
        let strip = &line[1..line.len() - 1];
        let nums: Vec<f32> = strip
            .split(", ")
            .map(|x| x.parse::<f32>().unwrap())
            .collect();

        // Adding numbers to matrix
        mat.add_edge(nums[0], nums[1], 0.0, nums[2], nums[3], 0.0);
    }

    // Returning matrix
    mat
}

/// Function that read lines from a file and returns a vector with the lines
pub fn read_lines(path: &str) -> Vec<String> {
    // Checking if file exists
    if (!file_exists(path)) {
        eprintln!("File {} not found, returning default value", path);
        return Vec::new();
    }

    // Getting data from the file
    let data = fs::read_to_string(path).expect("Unable to read data");
    let mut lines: Vec<String> = data.split('\n').map(|s| s.to_owned()).collect();

    // Exiting function
    lines.pop();
    lines
}

/// Function that displays an image file using built in open command
pub fn open_image(path: &str) {
    Command::new("display")
        .arg(path)
        .status()
        .expect("Open command failed to run");
}
