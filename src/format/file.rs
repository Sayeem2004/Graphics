// Imports
use crate::format::{image::Image, matrix::Matrix};
use std::io::{BufWriter, Write};
use std::process::Command;
use std::{cmp, fs};

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
    return fs::metadata(path).is_ok();
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
    let lines = data.split("\n");

    // Creating edge matrix
    let mut mat = Matrix::new_matrix();

    // Iterating through data and adding to matrix
    for line in lines {
        // Splitting into numbers
        if (line.len() == 0) {
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
    return mat;
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
    let mut lines: Vec<String> = data.split("\n").map(|s| s.to_owned()).collect();

    // Exiting function
    lines.pop();
    return lines;
}

/// Function that displays an image file using built in open command
pub fn open_image(path: &str) {
    Command::new("display")
        .arg(path)
        .status()
        .expect("Open command failed to run");
}

/// File that trims the number of lines from a csv file
pub fn trim_csv(path: &str, scale: i32) {
    // Getting original lines
    let mat: Matrix = read_lines_csv(path);
    let mut curr: usize = 0;
    let mut ret: Matrix = Matrix::new_matrix();

    // Transferring only certain lines
    while (curr < mat.col_num as usize) {
        let mut i: usize = 1;
        while (i < scale.abs() as usize) {
            if (curr + i + 2 < mat.col_num as usize
                && mat.data[curr + i][0] == mat.data[curr + i + 1][0]
                && mat.data[curr + i][1] == mat.data[curr + i + 1][1])
            {
                i += 2;
            } else {
                break;
            }
        }
        ret.add_edge(
            mat.data[curr][0],
            mat.data[curr][1],
            0.0,
            mat.data[cmp::min(curr + i, (mat.col_num - 1) as usize)][0],
            mat.data[cmp::min(curr + i, (mat.col_num - 1) as usize)][1],
            0.0,
        );
        curr += i + 1;
    }

    println!("Before: {}", mat.col_num);
    println!("After: {}", ret.col_num);

    // Attempting to create file and writer
    let file = fs::File::create("data/compressed.csv").expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    let mut curr: usize = 0;

    // Iterating through values and writing data
    while (curr < ret.col_num as usize) {
        // Joining numbers
        let nums = vec![
            ret.data[curr][0].to_string(),
            ret.data[curr][1].to_string(),
            ret.data[cmp::min(curr + 1, (ret.col_num - 1) as usize)][0].to_string(),
            ret.data[cmp::min(curr + 1, (ret.col_num - 1) as usize)][1].to_string(),
        ]
        .join(", ");

        // Writing data
        writer
            .write_all(
                ["[".to_string(), nums, "]".to_string(), "\n".to_string()]
                    .join("")
                    .as_bytes(),
            )
            .expect("Unable to write data");

        // Incrementing
        curr += 2;
    }

    // Ending message
    println!("CSV file is named data/compressed.csv");
}

/// Parsing lines from a csv into a script
pub fn convert_script(path: &str, size: i32) {
    // Checking if file exists
    if (!file_exists(path)) {
        eprintln!("File {} not found", path);
        return;
    }

    // Checking if image size is positive
    if (size <= 0) {
        eprintln!("Image size of {} is not possible", size);
        return;
    }

    // Getting data from csv file and splitting it
    let data = fs::read_to_string(path).expect("Unable to read data");
    let lines = data.split("\n");

    // Attempting to create script file and writer
    let file = fs::File::create("data/script").expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    // Iterating through data and adding to file
    for line in lines {
        // Splitting into numbers
        if (line.len() == 0) {
            continue;
        }
        let strip = &line[1..line.len() - 1];
        let nums: Vec<f32> = strip
            .split(", ")
            .map(|x| x.parse::<f32>().unwrap())
            .collect();

        // Adding numbers to new file
        writer
            .write_all(
                format!(
                    "line\n{} {} 0 {} {} 0\n",
                    nums[0],
                    size as f32 - nums[1],
                    nums[2],
                    size as f32 - nums[3]
                )
                .as_bytes(),
            )
            .expect("Unable to write data");
    }
    // Ending message
    println!("Script file is named data/script");
}
