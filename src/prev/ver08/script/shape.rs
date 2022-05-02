// Imports
use crate::prev::ver08::algorithm::shape;
use crate::prev::ver08::format::{constant, image::Image, matrix::Matrix};

/// Function that performs the 'box' command
pub fn _box(arg: &str, ind: usize, cord: &Matrix, img: &mut Image) {
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
                    "A \'box\' argument found at line {} is not a number",
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
    if (nums.len() != 6) {
        eprintln!(
            "\'box\' expected 6 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding box to image
    let mut poly: Matrix = Matrix::new_matrix();
    shape::add_box(
        &mut poly,
        (nums[0], nums[1], nums[2]),
        nums[3],
        nums[4],
        nums[5],
    );
    poly.left_transform(cord);
    poly.draw_triangles_xy(img, &constant::ZVIEW);
}

/// Function that performs the 'sphere' command
pub fn sphere(arg: &str, ind: usize, cord: &Matrix, img: &mut Image) {
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
                    "A \'sphere\' argument found at line {} is not a number",
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
            "\'sphere\' expected 4 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding sphere to image
    let mut poly: Matrix = Matrix::new_matrix();
    shape::add_sphere(&mut poly, (nums[0], nums[1], nums[2]), nums[3], 16);
    poly.left_transform(cord);
    poly.draw_triangles_xy(img, &constant::ZVIEW);
}

/// Function that performs the 'torus' command
pub fn torus(arg: &str, ind: usize, cord: &Matrix, img: &mut Image) {
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
                    "A \'torus\' argument found at line {} is not a number",
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
    if (nums.len() != 5) {
        eprintln!(
            "\'torus\' expected 5 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding torus to image
    let mut poly: Matrix = Matrix::new_matrix();
    shape::add_torus(&mut poly, (nums[0], nums[1], nums[2]), nums[3], nums[4], 16);
    poly.left_transform(cord);
    poly.draw_triangles_xy(img, &constant::ZVIEW);
}
