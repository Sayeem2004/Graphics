// Imports
use crate::algorithm::shape;
use crate::format::matrix::Matrix;

/// Function that performs the 'box' command
pub fn _box(arg: &str, ind: usize, poly: &mut Matrix) {
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

    // Adding box to matrix
    shape::add_box(poly, (nums[0], nums[1], nums[2]), nums[3], nums[4], nums[5]);
}

/// Function that performs the 'sphere' command
pub fn sphere(arg: &str, ind: usize, poly: &mut Matrix) {
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

    // Adding sphere to matrix
    shape::add_sphere(poly, (nums[0], nums[1], nums[2]), nums[3], 20);
}

/// Function that performs the 'torus' command
pub fn torus(arg: &str, ind: usize, poly: &mut Matrix) {
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

    // Adding torus to matrix
    shape::add_torus(poly, (nums[0], nums[1], nums[2]), nums[3], nums[4], 20);
}
