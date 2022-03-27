// Imports
use crate::algorithm::curve;
use crate::format::matrix::Matrix;

/// Function that performs the 'bezier' command
pub fn bezier(arg: &String, ind: usize, edge: &mut Matrix) {
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
                    "A \'bezier\' argument found at line {} is not a number",
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
    if (nums.len() != 8) {
        eprintln!(
            "\'bezier\' expected 8 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding bezier to matrix
    curve::add_bezier(
        edge, nums[0], nums[1], nums[2], nums[3], nums[4], nums[5], nums[6], nums[7], 100,
    );
}

/// Function that performs the 'circle' command
pub fn circle(arg: &String, ind: usize, edge: &mut Matrix) {
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
                    "A \'circle\' argument found at line {} is not a number",
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
    if (nums.len() != 4) {
        eprintln!(
            "\'circle\' expected 4 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding circle to matrix
    curve::add_circle(edge, nums[0], nums[1], nums[2], nums[3], 100);
}

/// Function that performs the 'hermite' command
pub fn hermite(arg: &String, ind: usize, edge: &mut Matrix) {
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
                    "A \'hermite\' argument found at line {} is not a number",
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
    if (nums.len() != 8) {
        eprintln!(
            "\'hermite\' expected 8 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Adding hermite to matrix
    curve::add_hermite(
        edge, nums[0], nums[1], nums[2], nums[3], nums[4], nums[5], nums[6], nums[7], 100,
    );
}
