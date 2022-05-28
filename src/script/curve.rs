// Imports
use crate::prev::ver09::algorithm::curve;
use crate::prev::ver09::format::{constant, image::Image, matrix::Matrix};

/// Function that performs the 'bezier' command
pub fn bezier(arg: &str, ind: usize, cord: &Matrix, img: &mut Image) {
    // Splitting the argument string
    let split = arg.split(' ');

    // Checking if each argument is a number
    for str in split {
        let num = str.parse::<f32>();
        match num {
            Ok(_) => {
                continue;
            }
            Err(_) => {
                eprintln!(
                    "A \'bezier\' argument found at line {} is not a number",
                    ind + 1
                );
                return;
            }
        }
    }

    // Converting to floats
    let nums: Vec<f32> = arg
        .split(' ')
        .map(|x| x.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();

    // Checking if right number of floats is found
    if (nums.len() != 8) {
        eprintln!(
            "\'bezier\' expected 8 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding bezier to image
    let mut edge: Matrix = Matrix::new_matrix();
    curve::add_bezier(
        &mut edge,
        (nums[0], nums[1]),
        (nums[2], nums[3]),
        (nums[4], nums[5]),
        (nums[6], nums[7]),
        100,
    );
    edge.left_transform(cord);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'circle' command
pub fn circle(arg: &str, ind: usize, cord: &Matrix, img: &mut Image) {
    // Splitting the argument string
    let split = arg.split(' ');

    // Checking if each argument is a number
    for str in split {
        let num = str.parse::<f32>();
        match num {
            Ok(_) => {
                continue;
            }
            Err(_) => {
                eprintln!(
                    "A \'circle\' argument found at line {} is not a number",
                    ind + 1
                );
                return;
            }
        }
    }

    // Converting to floats
    let nums: Vec<f32> = arg
        .split(' ')
        .map(|x| x.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();

    // Checking if right number of floats is found
    if (nums.len() != 4) {
        eprintln!(
            "\'circle\' expected 4 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding circle to image
    let mut edge: Matrix = Matrix::new_matrix();
    curve::add_circle(&mut edge, (nums[0], nums[1], nums[2]), nums[3], 100);
    edge.left_transform(cord);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'hermite' command
pub fn hermite(arg: &str, ind: usize, cord: &Matrix, img: &mut Image) {
    // Splitting the argument string
    let split = arg.split(' ');

    // Checking if each argument is a number
    for str in split {
        let num = str.parse::<f32>();
        match num {
            Ok(_) => {
                continue;
            }
            Err(_) => {
                eprintln!(
                    "A \'hermite\' argument found at line {} is not a number",
                    ind + 1
                );
                return;
            }
        }
    }

    // Converting to floats
    let nums: Vec<f32> = arg
        .split(' ')
        .map(|x| x.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();

    // Checking if right number of floats is found
    if (nums.len() != 8) {
        eprintln!(
            "\'hermite\' expected 8 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding hermite to image
    let mut edge: Matrix = Matrix::new_matrix();
    curve::add_hermite(
        &mut edge,
        (nums[0], nums[1]),
        (nums[2], nums[3]),
        (nums[4], nums[5]),
        (nums[6], nums[7]),
        100,
    );
    edge.left_transform(cord);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}
