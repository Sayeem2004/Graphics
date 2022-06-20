// Imports
use crate::template::prev::ver06::format::matrix::Matrix;

/// Function that performs the 'apply' command
pub fn apply(trans: &mut Matrix, edge: &mut Matrix, poly: &mut Matrix) {
    // Applying transformation
    edge.matrix_transform(trans);
    poly.matrix_transform(trans);
}

/// Function that performs the 'ident' command
pub fn ident(trans: &mut Matrix) {
    // Seting new value
    *trans = Matrix::new_transformation();
}

/// Function that performs the 'move' command
pub fn _move(arg: &str, ind: usize, trans: &mut Matrix) {
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
    trans.translate(nums[0], nums[1], nums[2]);
}

/// Function that performs the 'rotate' command
pub fn rotate(arg: &str, ind: usize, trans: &mut Matrix) {
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
    trans.rotate_degree(num.unwrap(), split[0]);
}

/// Function that performs the 'scale' command
pub fn scale(arg: &str, ind: usize, trans: &mut Matrix) {
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
    trans.dilate(nums[0], nums[1], nums[2]);
}
