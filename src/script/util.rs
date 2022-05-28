// Imports
use crate::format::{constant, file, image::Image, matrix::Matrix};
use crate::script::compile::{Argument, Operation, Symbol};
use rand::Rng;
use std::{collections::HashMap, fs, process::Command};

/// Function that returns the image to default black and clears the zbuffer
pub fn clear(img: &mut Image) {
    // Making the screen black
    img.fill(constant::BLACK_PIXEL);

    // Resetting zbuffer
    img.reset_buff();
}

/// Function that checks that a certain constant exists in the symbol table
pub fn constants(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>) {
    // Getting constants name
    let name: &String = op.constants.as_ref().unwrap();

    // Checking if it is inside the symbol table
    if (!symbols.contains_key(name)) {
        eprintln!("Constant {} not found in the symbol table", name);
    }
}

/// Function that performs the 'display' operation
pub fn display(img: &Image, mode: i32) {
    // Error checking mode
    if (mode != 0) {
        return;
    }

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

/// Function that performs the 'line' operation
pub fn line(op: &Operation, cord: &Matrix, img: &mut Image) {
    // Getting edge matrix
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut edge: Matrix = Matrix::new_matrix();
    edge.add_edge(
        (
            *args[0].as_float().unwrap(),
            *args[1].as_float().unwrap(),
            *args[2].as_float().unwrap(),
        ),
        (
            *args[3].as_float().unwrap(),
            *args[4].as_float().unwrap(),
            *args[5].as_float().unwrap(),
        ),
    );

    // Transforming matrix and drawing line
    edge.left_transform(cord);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'pop' operation
pub fn pop(stack: &mut Vec<Matrix>, sz: &mut usize) {
    // Removing top of stack
    stack.pop();

    // Updating stack size
    *sz -= 1;
}

/// Function that performs the 'push' operation
pub fn push(stack: &mut Vec<Matrix>, sz: &mut usize) {
    // Making copy of top
    let copy: Matrix = stack[*sz - 1].clone();

    // Adding copy to top of stack
    stack.push(copy);

    // Updating stack size
    *sz += 1;
}

/// Function that performs the 'save' operation
pub fn save(op: &Operation, img: &Image) {
    // Attempting to create image directory
    fs::create_dir_all("temp").expect("Unable to create temp directory");
    fs::create_dir_all("image/script").expect("Unable to create image/script directory");

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

    // Getting image path
    let mut path = String::from("image/script/");
    let add = op.args.as_ref().unwrap()[0].as_string().unwrap();
    path.push_str(add);

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
