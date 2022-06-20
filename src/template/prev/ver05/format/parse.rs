// Imports
use crate::template::prev::ver05::algorithm::{curve, shape};
use crate::template::prev::ver05::format::{constant, file, image::Image, matrix::Matrix};
use rand::Rng;
use std::{fs, process::Command};

/// Function that parses a graphics script file and runs commands
pub fn parse(path: &str, size: i32, mode: i32) {
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
                if (mode == 0) {
                    display(&mut edges, &mut img);
                }
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
            // Clear command
            "clear" => {
                clear(&mut edges);
                curr += 1;
            }
            // Box command
            "box" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'box\' at line {}",
                        curr + 1
                    );
                } else {
                    _box(&lines[curr + 1], curr + 1, &mut edges);
                }
                curr += 2;
            }
            // Sphere command
            "sphere" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'sphere\' at line {}",
                        curr + 1
                    );
                } else {
                    sphere(&lines[curr + 1], curr + 1, &mut edges);
                }
                curr += 2;
            }
            // Torus command
            "torus" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'torus\' at line {}",
                        curr + 1
                    );
                } else {
                    torus(&lines[curr + 1], curr + 1, &mut edges);
                }
                curr += 2;
            }
            // Empty line case
            "" => curr += 1,
            // Default case
            _ => {
                if (!lines[curr][0..1].eq("#")) {
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
fn line(arg: &str, ind: usize, edges: &mut Matrix) {
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
                    "A \'line\' argument found at line {} is not a number",
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
    fs::create_dir_all("temp").expect("Unable to create temp directory");

    // Saving image
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(0..10000);
    let name = vec![
        "temp/display".to_string(),
        format!("{:0>#4}", num),
        ".ppm".to_string(),
    ]
    .join("");
    file::create_ppm_ascii(&name, img, 1);

    // Displaying image
    file::open_image(&name);

    // Removing temporary file
    fs::remove_file(&name).expect("Unable to delete temporary display file");
}

/// Function that performs the 'ident' command
fn ident(trans: &mut Matrix) {
    // Seting new value
    *trans = Matrix::new_transformation();
}

/// Function that performs the 'scale' command
fn scale(arg: &str, ind: usize, trans: &mut Matrix) {
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

/// Function that performs the 'apply' command
fn apply(trans: &mut Matrix, edges: &mut Matrix) {
    // Applying transformation
    edges.matrix_transform(trans);
}

/// Function that performs the 'move' command
fn _move(arg: &str, ind: usize, trans: &mut Matrix) {
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
fn rotate(arg: &str, ind: usize, trans: &mut Matrix) {
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

/// Function that performs the 'save' command
fn save(arg: &str, edges: &mut Matrix, img: &mut Image) {
    // Clearing image
    img.fill(constant::BLACK_PIXEL);

    // Drawing lines
    edges.draw_lines_xy(img, constant::WHITE_PIXEL);

    // Attempting to create image directory
    fs::create_dir_all("temp").expect("Unable to create temp directory");

    // Saving image
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(0..10000);
    let name = vec![
        "temp/save".to_string(),
        format!("{:0>#4}", num),
        ".ppm".to_string(),
    ]
    .join("");
    file::create_ppm_ascii(&name, img, 1);

    // Attempting to create image directory
    fs::create_dir_all("image/script").expect("Unable to create image/script directory");

    // Getting image path
    let mut path = String::from("image/script/");
    path.push_str(arg);

    // Performing image magick convert command
    Command::new("convert")
        .arg(&name)
        .arg(&path)
        .status()
        .expect("Convert command failed to run");
    println!("Image file is named {}", path);

    // Removing temporary file
    fs::remove_file(&name).expect("Unable to delete temporary save file");
}

/// Function that performs the 'circle' command
fn circle(arg: &str, ind: usize, edges: &mut Matrix) {
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

    // Adding circle to matrix
    curve::add_circle(edges, nums[0], nums[1], nums[2], nums[3], 100);
}

/// Function that performs the 'hermite' command
fn hermite(arg: &str, ind: usize, edges: &mut Matrix) {
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

    // Adding hermite to matrix
    curve::add_hermite(
        edges,
        (nums[0], nums[1]),
        (nums[2], nums[3]),
        (nums[4], nums[5]),
        (nums[6], nums[7]),
        100,
    );
}

/// Function that performs the 'bezier' command
fn bezier(arg: &str, ind: usize, edges: &mut Matrix) {
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

    // Adding bezier to matrix
    curve::add_bezier(
        edges,
        (nums[0], nums[1]),
        (nums[2], nums[3]),
        (nums[4], nums[5]),
        (nums[6], nums[7]),
        100,
    );
}

/// Function that performs the 'clear' command
fn clear(edges: &mut Matrix) {
    // Seting new value
    *edges = Matrix::new_matrix();
}

/// Function that performs the 'box' command
fn _box(arg: &str, ind: usize, edges: &mut Matrix) {
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
    shape::add_box(edges, nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
}

/// Function that performs the 'sphere' command
fn sphere(arg: &str, ind: usize, edges: &mut Matrix) {
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
    shape::add_sphere(edges, nums[0], nums[1], nums[2], nums[3], 50);
}

/// Function that performs the 'torus' command
fn torus(arg: &str, ind: usize, edges: &mut Matrix) {
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
    shape::add_torus(edges, nums[0], nums[1], nums[2], nums[3], nums[4], 50);
}
