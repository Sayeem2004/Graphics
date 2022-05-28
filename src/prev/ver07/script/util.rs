// Imports
use crate::prev::ver07::format::{constant, file, image::Image, matrix::Matrix};
use rand::Rng;
use std::{fs, process::Command};

/// Function that performs the 'display' command
pub fn display(img: &Image) {
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

/// Function that performs the 'line' command
pub fn line(arg: &str, ind: usize, cord: &Matrix, img: &mut Image) {
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

    // Adding edges to matrix and drawing on image
    let mut edge: Matrix = Matrix::new_matrix();
    edge.add_edge((nums[0], nums[1], nums[2]), (nums[3], nums[4], nums[5]));
    edge.left_transform(cord);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'pop' command
pub fn pop(stack: &mut Vec<Matrix>, sz: &mut usize) {
    // Removing top of stack
    stack.pop();

    // Updating stack size
    *sz -= 1;
}

/// Function that performs the 'push' command
pub fn push(stack: &mut Vec<Matrix>, sz: &mut usize) {
    // Making copy of top
    let copy: Matrix = stack[*sz - 1].clone();

    // Adding copy to top of stack
    stack.push(copy);

    // Updating stack size
    *sz += 1;
}

/// Function that performs the 'save' command
pub fn save(arg: &str, img: &Image) {
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
