// Imports
use crate::format::{constant, file, image::Image, matrix::Matrix};
use std::{fs, process::Command};

/// Function that performs the 'clear' command
pub fn clear(edge: &mut Matrix, poly: &mut Matrix) {
    // Setting new value
    *edge = Matrix::new_matrix();
    *poly = Matrix::new_matrix();
}

/// Function that performs the 'display' command
pub fn display(edge: &mut Matrix, poly: &mut Matrix, img: &mut Image) {
    // Clearing image
    img.fill(constant::BLACK_PIXEL);

    // Drawing lines and polygons
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
    poly.draw_triangles_xy(img, constant::WHITE_PIXEL);

    // Attempting to create image directory
    fs::create_dir_all("image/tmp").expect("Unable to create image/tmp directory");

    // Saving image
    file::create_ppm_ascii("image/tmp/display.ppm", img, 1);

    // Displaying image
    file::open_image("image/tmp/display.ppm");

    // Removing ppm file
    let res = fs::remove_dir_all("image/tmp");
    match res {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Unable to remove temporary image/tmp directory");
        }
    }
}

/// Function that performs the 'line' command
pub fn line(arg: &String, ind: usize, edge: &mut Matrix) {
    // Splitting the argument string
    let split = arg.split(" ");

    // Checking if each argument is a number
    for str in split {
        let num = str.parse::<f32>();
        match num {
            Ok(_) => {
                continue;
            }
            Err(_) => {
                eprintln!(
                    "A \'line\' argument found at line {} is not a number",
                    ind + 1
                );
                return;
            }
        }
    }

    // Converting to floats
    let nums: Vec<f32> = arg
        .split(" ")
        .map(|x| x.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();

    // Checking if right number of floats is found
    if (nums.len() != 6) {
        eprintln!(
            "\'line\' expected 6 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding edge to matrix
    edge.add_edge(nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
}

/// Function that performs the 'save' command
pub fn save(arg: &String, edge: &mut Matrix, poly: &mut Matrix, img: &mut Image) {
    // Clearing image
    img.fill(constant::BLACK_PIXEL);

    // Drawing lines and polygons
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
    poly.draw_triangles_xy(img, constant::WHITE_PIXEL);

    // Attempting to create image directory
    fs::create_dir_all("image/tmp").expect("Unable to create image/tmp directory");

    // Saving image
    file::create_ppm_ascii("image/tmp/save.ppm", img, 1);

    // Attempting to create image directory
    fs::create_dir_all("image/script").expect("Unable to create image/script directory");

    // Getting image path
    let mut path = String::from("image/script/");
    path.push_str(arg);

    // Performing image magick convert command
    Command::new("convert")
        .arg("image/tmp/save.ppm")
        .arg(&path)
        .status()
        .expect("Open command failed to run");
    println!("Image file is named {}", path);

    // Removing ppm file
    let res = fs::remove_dir_all("image/tmp");
    match res {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Unable to remove temporary image/tmp directory");
        }
    }
}
