// Imports
use crate::template::prev::ver08::format::matrix::Matrix;

/// Function that performs the 'move' command
pub fn _move(arg: &str, ind: usize, cord: &mut Matrix) {
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
                    "A \'move\' argument found at line {} is not a number",
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
    if (nums.len() != 3) {
        eprintln!(
            "\'move\' expected 3 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Performing translation
    let trans: Matrix = Matrix::translate(nums[0], nums[1], nums[2]);
    cord.right_transform(&trans);
}

/// Function that performs the 'rotate' command
pub fn rotate(arg: &str, ind: usize, cord: &mut Matrix) {
    // Splitting the argument string
    let split: Vec<&str> = arg.split(' ').collect();

    // Checking if rotation axis is valid
    match split[0] {
        "x" => (),
        "y" => (),
        "z" => (),
        _ => {
            eprintln!(
                "The 1st \'rotate\' argument fount at line {} is not x, y, or z",
                ind + 1
            );
            return;
        }
    }
    // Checking if each argument is a number
    let num = split[1].parse::<f32>();
    match num {
        Ok(_) => (),
        Err(_) => {
            eprintln!(
                "The 2nd \'rotate\' argument found at line {} is not a number",
                ind + 1
            );
            return;
        }
    }

    // Performing rotation
    let trans: Matrix = Matrix::rotate_degree(num.unwrap(), split[0]);
    cord.right_transform(&trans);
}

/// Function that performs the 'scale' command
pub fn scale(arg: &str, ind: usize, cord: &mut Matrix) {
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
                    "A \'scale\' argument found at line {} is not a number",
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
    if (nums.len() != 3) {
        eprintln!(
            "\'scale\' expected 3 numerical arguments, but {} were found",
            nums.len()
        );
        return;
    }

    // Performing dilation
    let trans: Matrix = Matrix::dilate(nums[0], nums[1], nums[2]);
    cord.right_transform(&trans);
}
