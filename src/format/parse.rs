// Imports
use crate::algorithm::curve;
use crate::format::{constant, file, image::Image, matrix::Matrix};
use std::{fs, process::Command};

/// Function that parses a graphics script file and runs commands
pub fn parse(path: &str, size: i32) {
    // Making sure the script file exists
    if (!file::file_exists(path)) {
        eprintln!("Script file does not exist, ending parsing attempt");
        return;
    }

    // Getting all lines from the script file
    let lines: Vec<String> = file::read_lines(path);
    let mx: usize = lines.len();
    let mut curr: usize = 0;

    // Creating transformation matrix, edge matrix, and image struct
    let mut edges: Matrix = Matrix::new_matrix();
    let mut trans: Matrix = Matrix::new_transformation();
    let mut img: Image = Image::new_dimension(size, size);

    // Looping through all lines
    while (curr < mx) {
        match lines[curr].as_str() {
            // Line command
            "line" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'line\' at line {}",
                        curr + 1
                    );
                } else {
                    line(&lines[curr + 1], curr + 1, &mut edges);
                }
                curr += 2;
            }
            // Display command
            "display" => {
                display(&mut edges, &mut img);
                curr += 1;
            }
            // Ident command
            "ident" => {
                ident(&mut trans);
                curr += 1;
            }
            // Scale command
            "scale" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'scale\' at line {}",
                        curr + 1
                    );
                } else {
                    scale(&lines[curr + 1], curr + 1, &mut trans);
                }
                curr += 2;
            }
            // Apply command
            "apply" => {
                apply(&mut trans, &mut edges);
                curr += 1;
            }
            // Move command
            "move" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'move\' at line {}",
                        curr + 1
                    );
                } else {
                    _move(&lines[curr + 1], curr + 1, &mut trans);
                }
                curr += 2;
            }
            // Rotate command
            "rotate" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'rotate\' at line {}",
                        curr + 1
                    );
                } else {
                    rotate(&lines[curr + 1], curr + 1, &mut trans);
                }
                curr += 2;
            }
            // Save command
            "save" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'save\' at line {}",
                        curr + 1
                    );
                } else {
                    save(&lines[curr + 1], &mut edges, &mut img);
                }
                curr += 2;
            }
            // Circle command
            "circle" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'circle\' at line {}",
                        curr + 1
                    );
                } else {
                    circle(&lines[curr + 1], curr + 1, &mut edges);
                }
                curr += 2;
            }
            // Hermite command
            "hermite" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'hermite\' at line {}",
                        curr + 1
                    );
                } else {
                    hermite(&lines[curr + 1], curr + 1, &mut edges);
                }
                curr += 2;
            }
            // Bezier command
            "bezier" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'bezier\' at line {}",
                        curr + 1
                    );
                } else {
                    bezier(&lines[curr + 1], curr + 1, &mut edges);
                }
                curr += 2;
            }
            // Empty line case
            "" => curr += 1,
            // Default case
            _ => {
                if (lines[curr][0..1].eq("#")) {
                    ();
                } else {
                    eprintln!(
                        "The command \'{}\' found at line {} is invalid",
                        lines[curr],
                        curr + 1
                    );
                }
                curr += 1;
            }
        }
    }
}

/// Function that performs the 'line' command
fn line(arg: &String, ind: usize, edges: &mut Matrix) {
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
    edges.add_edge(nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
}

/// Function that performs the 'display' command
fn display(edges: &mut Matrix, img: &mut Image) {
    // Clearing image
    img.fill(constant::BLACK_PIXEL);

    // Drawing lines
    edges.draw_lines_xy(img, constant::WHITE_PIXEL);

    // Attempting to create image directory
    fs::create_dir_all("image/tmp").expect("Unable to create image/tmp directory");

    // Saving image
    file::create_ppm_ascii("image/tmp/display.ppm", img);

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

/// Function that performs the 'ident' command
fn ident(trans: &mut Matrix) {
    // Seting new value
    *trans = Matrix::new_transformation();
}

/// Function that performs the 'scale' command
fn scale(arg: &String, ind: usize, trans: &mut Matrix) {
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
                    "A \'scale\' argument found at line {} is not a number",
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

/// Function that performs the 'apply' command
fn apply(trans: &mut Matrix, edges: &mut Matrix) {
    // Applying transformation
    edges.matrix_transform(trans);
}

/// Function that performs the 'move' command
fn _move(arg: &String, ind: usize, trans: &mut Matrix) {
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
                    "A \'move\' argument found at line {} is not a number",
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
fn rotate(arg: &String, ind: usize, trans: &mut Matrix) {
    // Splitting the argument string
    let split: Vec<&str> = arg.split(" ").collect();

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

/// Function that performs the 'save' command
fn save(arg: &String, edges: &mut Matrix, img: &mut Image) {
    // Clearing image
    img.fill(constant::BLACK_PIXEL);

    // Drawing lines
    edges.draw_lines_xy(img, constant::WHITE_PIXEL);

    // Attempting to create image directory
    fs::create_dir_all("image/tmp").expect("Unable to create image/tmp directory");

    // Saving image
    file::create_ppm_ascii("image/tmp/save.ppm", img);

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

/// Function that performs the 'circle' command
fn circle(arg: &String, ind: usize, edges: &mut Matrix) {
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
    curve::add_circle(edges, nums[0], nums[1], nums[2], nums[3], 0.02);
}

/// Function that performs the 'hermite' command
fn hermite(arg: &String, ind: usize, edges: &mut Matrix) {
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
        edges, nums[0], nums[1], nums[2], nums[3], nums[4], nums[5], nums[6], nums[7], 0.02,
    );
}

/// Function that performs the 'bezier' command
fn bezier(arg: &String, ind: usize, edges: &mut Matrix) {
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

    // Adding hermite to matrix
    curve::add_bezier(
        edges, nums[0], nums[1], nums[2], nums[3], nums[4], nums[5], nums[6], nums[7], 0.02,
    );
}
